use crate::constants::{
    DELAYED_CORRECTION_MAX_CHARS, DELAYED_CORRECTION_MIN_CHARS, IMMEDIATE_CORRECTION_SHARE,
};
use crate::rng::ClackRng;

pub enum CorrectionMode {
    Immediate,
    Delayed { chars_to_continue: usize },
}

pub fn select_correction_mode(rng: &mut ClackRng, correction_rate: f64) -> Option<CorrectionMode> {
    if !rng.sample_bool(correction_rate) {
        return None;
    }

    if rng.sample_bool(IMMEDIATE_CORRECTION_SHARE) {
        Some(CorrectionMode::Immediate)
    } else {
        let chars_to_continue =
            rng.sample_uniform_int(DELAYED_CORRECTION_MIN_CHARS, DELAYED_CORRECTION_MAX_CHARS);
        Some(CorrectionMode::Delayed { chars_to_continue })
    }
}

pub fn emit_immediate(rng: &mut ClackRng, correct_char: u8, retype_delay: u64) -> Vec<crate::ClackEvent> {
    use crate::constants::{
        CORRECTION_PAUSE_MAX_MS, CORRECTION_PAUSE_MIN_MS, CORRECTION_PAUSE_MU_MS,
        CORRECTION_PAUSE_SIGMA, CHAR_BY_CHAR_MAX_MS, CHAR_BY_CHAR_MIN_MS,
    };
    let mut events = Vec::new();

    let sigma = CORRECTION_PAUSE_SIGMA;
    let mu = CORRECTION_PAUSE_MU_MS.ln() - (sigma * sigma / 2.0);
    let notice_pause = rng
        .sample_log_normal(mu, sigma)
        .clamp(CORRECTION_PAUSE_MIN_MS, CORRECTION_PAUSE_MAX_MS) as u64;

    let bs_delay = rng.sample_uniform(CHAR_BY_CHAR_MIN_MS, CHAR_BY_CHAR_MAX_MS) as u64;

    events.push(crate::ClackEvent {
        delay_ms: notice_pause + bs_delay,
        bytes: vec![0x08],
        state_transition: None,
    });

    events.push(crate::ClackEvent {
        delay_ms: retype_delay,
        bytes: vec![correct_char],
        state_transition: None,
    });

    events
}
pub fn emit_delayed(rng: &mut ClackRng, num_backspaces: usize) -> Vec<crate::ClackEvent> {
    use crate::constants::{
        NOTICE_PAUSE_MAX_MS, NOTICE_PAUSE_MIN_MS,
        CHAR_BY_CHAR_BACKSPACE_PROB,
        CHAR_BY_CHAR_MAX_MS, CHAR_BY_CHAR_MIN_MS,
        HELD_BACKSPACE_MAX_MS, HELD_BACKSPACE_MIN_MS,
    };
    let mut events = Vec::new();

    let notice_pause = rng.sample_uniform(NOTICE_PAUSE_MIN_MS, NOTICE_PAUSE_MAX_MS) as u64;
    let is_char_by_char = rng.sample_bool(CHAR_BY_CHAR_BACKSPACE_PROB);

    for i in 0..num_backspaces {
        let bs_delay = if is_char_by_char {
            rng.sample_uniform(CHAR_BY_CHAR_MIN_MS, CHAR_BY_CHAR_MAX_MS) as u64
        } else {
            rng.sample_uniform(HELD_BACKSPACE_MIN_MS, HELD_BACKSPACE_MAX_MS) as u64
        };

        let total_delay = if i == 0 { notice_pause + bs_delay } else { bs_delay };

        events.push(crate::ClackEvent {
            delay_ms: total_delay,
            bytes: vec![0x08],
            state_transition: None,
        });
    }

    events
}
