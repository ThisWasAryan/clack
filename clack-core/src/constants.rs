// Timing
pub const MOMENTUM_FACTOR: f64 = 0.35;
pub const IKI_HARD_FLOOR_MS: f64 = 60.0;
pub const JITTER_SIGMA_SCALE: f64 = 0.40;

// Pauses
pub const WORD_PAUSE_MU_MS: f64 = 80.0;
pub const WORD_PAUSE_SIGMA: f64 = 0.30;
pub const WORD_PAUSE_MIN_MS: f64 = 30.0;
pub const WORD_PAUSE_MAX_MS: f64 = 300.0;

pub const SENTENCE_PAUSE_MU_MS: f64 = 600.0;
pub const SENTENCE_PAUSE_SIGMA: f64 = 0.40;
pub const SENTENCE_PAUSE_MIN_MS: f64 = 200.0;
pub const SENTENCE_PAUSE_MAX_MS: f64 = 2000.0;

pub const LINE_START_MU_MS: f64 = 200.0;
pub const LINE_START_SIGMA: f64 = 0.45;
pub const LINE_START_MIN_MS: f64 = 80.0;
pub const LINE_START_MAX_MS: f64 = 800.0;

// Bursts
pub const BURST_TRIGGER_PROB: f64 = 0.08;
pub const BURST_DURATION_MIN_CHARS: usize = 8;
pub const BURST_DURATION_MAX_CHARS: usize = 25;
pub const BURST_SPEED_MULTIPLIER: f64 = 0.65;
pub const POST_BURST_PAUSE_MIN_MS: f64 = 200.0;
pub const POST_BURST_PAUSE_MAX_MS: f64 = 600.0;

// Keyboard
pub const SHIFT_PENALTY_MU_MS: f64 = 45.0;
pub const SHIFT_PENALTY_SIGMA: f64 = 0.30;
pub const SHIFT_PENALTY_MIN_MS: f64 = 20.0;
pub const SHIFT_PENALTY_MAX_MS: f64 = 120.0;

// Corrections
pub const DELAYED_CORRECTION_MIN_CHARS: usize = 3;
pub const DELAYED_CORRECTION_MAX_CHARS: usize = 12;

pub const CHAR_BY_CHAR_BACKSPACE_PROB: f64 = 0.65;
pub const IMMEDIATE_CORRECTION_SHARE: f64 = 0.70;

pub const BACKSPACE_IKI_MULT_MIN: f64 = 0.6;
pub const BACKSPACE_IKI_MULT_MAX: f64 = 1.0;
pub const HELD_BACKSPACE_IKI_MULT_MIN: f64 = 0.3;
pub const HELD_BACKSPACE_IKI_MULT_MAX: f64 = 0.5;

// Language
pub const COMMON_WORD_MULTIPLIER: f64 = 0.80;
pub const DIFFICULT_WORD_MULTIPLIER: f64 = 1.25;
pub const DIFFICULT_WORD_MIN_LENGTH: usize = 9;

// Session
pub const WARMUP_FRACTION: f64 = 0.10;
pub const FATIGUE_START_FRACTION: f64 = 0.60;
pub const WARMUP_SPEED_PENALTY: f64 = 0.30;
pub const FATIGUE_MAX_SPEED_PENALTY: f64 = 0.25;
pub const FATIGUE_MAX_ERROR_PENALTY: f64 = 0.50;
pub const FATIGUE_LAPSE_START: f64 = 0.70;
pub const FATIGUE_LAPSE_PROB: f64 = 0.03;
pub const FATIGUE_LAPSE_MIN_MS: f64 = 1500.0;
pub const FATIGUE_LAPSE_MAX_MS: f64 = 4000.0;

// Error rate cap
pub const ERROR_RATE_MAX_EFFECTIVE: f64 = 0.35;
