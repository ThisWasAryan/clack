use crate::constants::*;
use crate::rng::ClackRng;

pub enum PauseType {
    Word,
    Sentence,
    LineStart,
}

pub fn compute_pause(rng: &mut ClackRng, pause_type: PauseType) -> f64 {
    match pause_type {
        PauseType::Word => {
            let sigma = WORD_PAUSE_SIGMA;
            let mu = WORD_PAUSE_MU_MS.ln() - (sigma * sigma / 2.0);
            rng.sample_log_normal(mu, sigma).clamp(WORD_PAUSE_MIN_MS, WORD_PAUSE_MAX_MS)
        }
        PauseType::Sentence => {
            let sigma = SENTENCE_PAUSE_SIGMA;
            let mu = SENTENCE_PAUSE_MU_MS.ln() - (sigma * sigma / 2.0);
            rng.sample_log_normal(mu, sigma).clamp(SENTENCE_PAUSE_MIN_MS, SENTENCE_PAUSE_MAX_MS)
        }
        PauseType::LineStart => {
            let sigma = LINE_START_SIGMA;
            let mu = LINE_START_MU_MS.ln() - (sigma * sigma / 2.0);
            rng.sample_log_normal(mu, sigma).clamp(LINE_START_MIN_MS, LINE_START_MAX_MS)
        }
    }
}
