#![allow(dead_code, unused_variables)]

pub mod constants;
pub mod correction;
pub mod error;
pub mod keyboard;
pub mod language;
pub mod pause;
pub mod rng;
pub mod session;
pub mod state;
pub mod timing;
pub mod tokenizer;

#[non_exhaustive]
pub struct ClackConfig {
    pub wpm: f64,
    pub jitter: f64,
    pub error_rate: f64,
    pub correction_rate: f64,
    pub no_errors: bool,
    pub seed: Option<u64>,
    pub session_length: usize,
    pub no_fatigue: bool,
    pub max_pause_ms: u64,
    pub thinking_pause_prob: f64,
    pub state_output: bool,
}

impl Default for ClackConfig {
    fn default() -> Self {
        Self {
            wpm: 60.0,
            jitter: 0.15,
            error_rate: 0.04,
            correction_rate: 0.85,
            no_errors: false,
            seed: None,
            session_length: 500,
            no_fatigue: false,
            max_pause_ms: 5000,
            thinking_pause_prob: 0.015,
            state_output: false,
        }
    }
}

#[non_exhaustive]
pub struct ClackEvent {
    pub delay_ms: u64,
    pub bytes: Vec<u8>,
    pub state_transition: Option<StateTransition>,
}

pub struct StateTransition {
    pub prev_state: BehavioralState,
    pub new_state: BehavioralState,
    pub word_count: usize,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehavioralState {
    Focused,
    Flow,
    Thinking,
    Distracted,
    Fatigued,
}

impl std::fmt::Display for BehavioralState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Focused => write!(f, "FOCUSED"),
            Self::Flow => write!(f, "FLOW"),
            Self::Thinking => write!(f, "THINKING"),
            Self::Distracted => write!(f, "DISTRACTED"),
            Self::Fatigued => write!(f, "FATIGUED"),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidWpm(f64),
    InvalidJitter(f64),
    InvalidErrorRate(f64),
    InvalidCorrectionRate(f64),
    InvalidMaxPause(u64),
}

pub struct ClackEngine {
    pub config: ClackConfig,
    rng: rng::ClackRng,
    tokenizer: tokenizer::Tokenizer,
    event_queue: std::collections::VecDeque<ClackEvent>,

    iki_prev: f64,
    prev_pos: Option<(i32, i32)>,
    burst_state: timing::BurstState,
    state_manager: state::StateManager,
    chars_typed: usize,
    accumulated_pause: u64,
}

impl ClackEngine {
    pub fn new(config: ClackConfig) -> Result<Self, ConfigError> {
        if config.wpm <= 0.0 {
            return Err(ConfigError::InvalidWpm(config.wpm));
        }
        let rng = rng::ClackRng::new(config.seed);
        let tokenizer = tokenizer::Tokenizer::new();
        let iki_prev = timing::compute_base_iki(config.wpm);

        Ok(Self {
            config,
            rng,
            tokenizer,
            event_queue: std::collections::VecDeque::new(),
            iki_prev,
            prev_pos: None,
            burst_state: timing::BurstState::new(),
            state_manager: state::StateManager::new(),
            chars_typed: 0,
            accumulated_pause: 0,
        })
    }

    pub fn feed(&mut self, input: &[u8]) {
        self.tokenizer.feed(input);
        self.process_ready_words();
    }

    fn process_ready_words(&mut self) {
        while let Some(idx) = self.tokenizer.buffer.find([' ', '\n', '\t']) {
            let word: String = self.tokenizer.buffer.drain(..=idx).collect();
            self.process_word(&word);
        }
    }

    fn process_word(&mut self, word: &str) {
        let is_sentence_boundary = tokenizer::Tokenizer::is_sentence_boundary(word.trim_end());
        let is_difficult = word.trim_end().chars().count() >= constants::DIFFICULT_WORD_MIN_LENGTH;

        self.burst_state.try_trigger(&mut self.rng, true);

        let progress = session::session_progress(self.chars_typed, self.config.session_length);
        let warmup_mod = if self.config.no_fatigue { 1.0 } else { session::warmup_multiplier(progress) };
        let (fatigue_speed_mod, fatigue_error_mod) = if self.config.no_fatigue { (1.0, 1.0) } else { session::fatigue_multiplier(progress) };

        let mut state_transition = self.state_manager.try_transition(
            &mut self.rng,
            is_sentence_boundary,
            is_difficult,
            progress > constants::FATIGUE_START_FRACTION,
            false,
        );

        let word_mod = language::apply_word_modifier(1.0, word.trim_end());

        let base_iki = timing::compute_base_iki(self.config.wpm);
        let target_iki = base_iki * word_mod * warmup_mod * fatigue_speed_mod;
        
        let chars: Vec<char> = word.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let c = chars[i];
            self.chars_typed += 1;
            
            let effective_error_rate = if self.config.no_errors { 0.0 } else { self.config.error_rate * fatigue_error_mod };
            let error_occurs = error::should_generate_error(&mut self.rng, effective_error_rate);
            
            if error_occurs {
                let error_type = error::select_error_type(&mut self.rng);
                let correction_mode = correction::select_correction_mode(&mut self.rng, self.config.correction_rate);
                
                let wrong_chars = match error_type {
                    error::ErrorType::Typo => vec![error::generate_typo(&mut self.rng, c)],
                    error::ErrorType::Transposition => {
                        if i + 1 < chars.len() {
                            let v = vec![chars[i+1], c];
                            i += 1;
                            v
                        } else {
                            vec![error::generate_typo(&mut self.rng, c)]
                        }
                    },
                    error::ErrorType::Omission => vec![],
                };

                for wc in &wrong_chars {
                    let iki_final = self.compute_char_iki(target_iki, *wc);
                    self.queue_event(*wc as u8, iki_final, state_transition.take());
                }

                if let Some(mode) = correction_mode {
                    match mode {
                        correction::CorrectionMode::Immediate => {
                            let correct_char = c;
                            let events = correction::emit_immediate(&mut self.rng, correct_char as u8, base_iki as u64);
                            for mut e in events {
                                e.state_transition = state_transition.take();
                                self.event_queue.push_back(e);
                            }
                        }
                        correction::CorrectionMode::Delayed { chars_to_continue } => {
                            let events = correction::emit_delayed(&mut self.rng, wrong_chars.len());
                            for mut e in events {
                                e.state_transition = state_transition.take();
                                self.event_queue.push_back(e);
                            }
                            for &correct in &chars[(i.saturating_sub(wrong_chars.len().max(1) - 1))..=i] {
                                let iki_final = self.compute_char_iki(target_iki, correct);
                                self.queue_event(correct as u8, iki_final, state_transition.take());
                            }
                        }
                    }
                }
            } else {
                let iki_final = self.compute_char_iki(target_iki, c);
                self.queue_event(c as u8, iki_final, state_transition.take());
            }

            if let Some(post_burst) = self.burst_state.advance_char(&mut self.rng) {
                self.accumulated_pause += post_burst;
            }
            
            i += 1;
        }
        
        self.state_manager.advance_word();

        let mut pause = 0.0;
        if is_sentence_boundary {
            pause += pause::compute_pause(&mut self.rng, pause::PauseType::Sentence);
        } else {
            pause += pause::compute_pause(&mut self.rng, pause::PauseType::Word);
        }

        if let Some(lapse) = session::check_lapse(&mut self.rng, progress, true) {
            pause += lapse as f64;
        }

        self.accumulated_pause += pause as u64;
    }
    
    fn compute_char_iki(&mut self, target_iki: f64, c: char) -> u64 {
        let mut iki_raw = timing::sample_raw_iki(&mut self.rng, target_iki, self.config.jitter);
        iki_raw = self.burst_state.apply_modifier(iki_raw);
        iki_raw = timing::apply_hard_floor(iki_raw);

        let curr_pos = keyboard::key_position(c);
        if let (Some(p1), Some(p2)) = (self.prev_pos, curr_pos) {
            iki_raw = keyboard::apply_distance_modifier(iki_raw, p1, p2);
            iki_raw = keyboard::apply_hand_modifier(iki_raw, p1, p2);
        }
        if c.is_ascii_uppercase() || "!@#$%^&*()_+{}|:\"<>?~".contains(c) {
            iki_raw += keyboard::shift_penalty(&mut self.rng);
        }

        let iki_final = timing::apply_momentum(self.iki_prev, iki_raw);
        self.iki_prev = iki_final;
        self.prev_pos = curr_pos;
        iki_final as u64
    }

    fn queue_event(&mut self, byte: u8, delay: u64, transition: Option<StateTransition>) {
        let total_delay = delay + self.accumulated_pause;
        self.accumulated_pause = 0;
        self.event_queue.push_back(ClackEvent {
            delay_ms: total_delay,
            bytes: vec![byte],
            state_transition: transition,
        });
    }

    pub fn next_event(&mut self) -> Option<ClackEvent> {
        self.event_queue.pop_front()
    }

    pub fn finish(&mut self) {
        let remaining = std::mem::take(&mut self.tokenizer.buffer);
        if !remaining.is_empty() {
            self.process_word(&remaining);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_init() {
        let config = ClackConfig::default();
        let mut engine = ClackEngine::new(config).unwrap_or_else(|_| panic!("Config error"));
        engine.finish();
    }

    #[test]
    fn test_processing_performance() {
        use std::time::Instant;
        let mut config = ClackConfig::default();
        config.wpm = 120.0;
        let mut engine = ClackEngine::new(config).unwrap();
        
        let input = "hello world ".repeat(100);
        let start = Instant::now();
        engine.feed(input.as_bytes());
        engine.finish();
        
        let mut event_count = 0;
        while let Some(_) = engine.next_event() {
            event_count += 1;
        }
        
        let duration = start.elapsed();
        let ms_per_char = duration.as_secs_f64() * 1000.0 / event_count as f64;
        assert!(ms_per_char < 1.0, "Processing too slow: {} ms/char", ms_per_char);
    }
}
