use crate::constants::*;
use crate::rng::ClackRng;

pub fn session_progress(current_chars: usize, session_length: usize) -> f64 {
    if session_length == 0 {
        return 0.0;
    }
    (current_chars as f64) / (session_length as f64)
}

pub fn warmup_multiplier(progress: f64) -> f64 {
    if progress < WARMUP_FRACTION {
        let completion = progress / WARMUP_FRACTION;
        let penalty = WARMUP_SPEED_PENALTY * (1.0 - completion);
        1.0 + penalty
    } else {
        1.0
    }
}

pub fn fatigue_multiplier(progress: f64) -> (f64, f64) {
    if progress > FATIGUE_START_FRACTION {
        let completion = (progress - FATIGUE_START_FRACTION) / (1.0 - FATIGUE_START_FRACTION);
        let completion_clamped = completion.clamp(0.0, 1.0);
        let speed_penalty = FATIGUE_MAX_SPEED_PENALTY * completion_clamped;
        let error_penalty = FATIGUE_MAX_ERROR_PENALTY * completion_clamped;
        (1.0 + speed_penalty, 1.0 + error_penalty)
    } else {
        (1.0, 1.0)
    }
}

pub fn check_lapse(rng: &mut ClackRng, progress: f64, is_word_boundary: bool) -> Option<u64> {
    if is_word_boundary && progress >= FATIGUE_LAPSE_START && rng.sample_bool(FATIGUE_LAPSE_PROB) {
        return Some(rng.sample_uniform(FATIGUE_LAPSE_MIN_MS, FATIGUE_LAPSE_MAX_MS) as u64);
    }
    None
}
