use crate::constants::{
    BURST_DURATION_MAX_CHARS, BURST_DURATION_MIN_CHARS, BURST_SPEED_MULTIPLIER, BURST_TRIGGER_PROB,
    IKI_HARD_FLOOR_MS, JITTER_SIGMA_SCALE, MOMENTUM_FACTOR, POST_BURST_PAUSE_MAX_MS,
    POST_BURST_PAUSE_MIN_MS,
};
use crate::rng::ClackRng;

pub fn compute_base_iki(wpm: f64) -> f64 {
    60_000.0 / (wpm * 5.0)
}

pub fn sample_raw_iki(rng: &mut ClackRng, iki_target: f64, jitter: f64) -> f64 {
    let sigma = jitter * JITTER_SIGMA_SCALE;
    let mu = iki_target.ln() - (sigma * sigma / 2.0);
    rng.sample_log_normal(mu, sigma)
}

pub fn apply_hard_floor(iki: f64) -> f64 {
    iki.max(IKI_HARD_FLOOR_MS)
}

pub fn apply_momentum(iki_prev: f64, iki_raw: f64) -> f64 {
    (MOMENTUM_FACTOR * iki_prev) + ((1.0 - MOMENTUM_FACTOR) * iki_raw)
}

pub struct BurstState {
    pub active: bool,
    pub chars_remaining: usize,
}

impl Default for BurstState {
    fn default() -> Self {
        Self::new()
    }
}

impl BurstState {
    pub fn new() -> Self {
        Self {
            active: false,
            chars_remaining: 0,
        }
    }

    pub fn try_trigger(&mut self, rng: &mut ClackRng, is_word_start: bool) {
        if !self.active && is_word_start && rng.sample_bool(BURST_TRIGGER_PROB) {
            self.active = true;
            self.chars_remaining = rng
                .sample_uniform_int(BURST_DURATION_MIN_CHARS, BURST_DURATION_MAX_CHARS);
        }
    }

    pub fn apply_modifier(&self, iki_raw: f64) -> f64 {
        if self.active {
            iki_raw * BURST_SPEED_MULTIPLIER
        } else {
            iki_raw
        }
    }

    pub fn advance_char(&mut self, rng: &mut ClackRng) -> Option<u64> {
        if self.active {
            self.chars_remaining = self.chars_remaining.saturating_sub(1);
            if self.chars_remaining == 0 {
                self.active = false;
                return Some(
                    rng.sample_uniform(POST_BURST_PAUSE_MIN_MS, POST_BURST_PAUSE_MAX_MS) as u64,
                );
            }
        }
        None
    }
}
