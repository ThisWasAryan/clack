use crate::constants::*;
use crate::rng::ClackRng;

pub fn key_position(c: char) -> Option<(i32, i32)> {
    let c = c.to_ascii_lowercase();
    match c {
        // Row 0
        '1' => Some((0, 0)), '2' => Some((1, 0)), '3' => Some((2, 0)), '4' => Some((3, 0)),
        '5' => Some((4, 0)), '6' => Some((5, 0)), '7' => Some((6, 0)), '8' => Some((7, 0)),
        '9' => Some((8, 0)), '0' => Some((9, 0)), '-' => Some((10, 0)), '=' => Some((11, 0)),
        // Row 1
        'q' => Some((0, 1)), 'w' => Some((1, 1)), 'e' => Some((2, 1)), 'r' => Some((3, 1)),
        't' => Some((4, 1)), 'y' => Some((5, 1)), 'u' => Some((6, 1)), 'i' => Some((7, 1)),
        'o' => Some((8, 1)), 'p' => Some((9, 1)), '[' => Some((10, 1)), ']' => Some((11, 1)),
        // Row 2
        'a' => Some((0, 2)), 's' => Some((1, 2)), 'd' => Some((2, 2)), 'f' => Some((3, 2)),
        'g' => Some((4, 2)), 'h' => Some((5, 2)), 'j' => Some((6, 2)), 'k' => Some((7, 2)),
        'l' => Some((8, 2)), ';' => Some((9, 2)), '\'' => Some((10, 2)),
        // Row 3
        'z' => Some((0, 3)), 'x' => Some((1, 3)), 'c' => Some((2, 3)), 'v' => Some((3, 3)),
        'b' => Some((4, 3)), 'n' => Some((5, 3)), 'm' => Some((6, 3)), ',' => Some((7, 3)),
        '.' => Some((8, 3)), '/' => Some((9, 3)),
        // Row 4
        ' ' => Some((5, 4)),
        _ => None,
    }
}

pub fn distance(pos1: (i32, i32), pos2: (i32, i32)) -> f64 {
    let dx = (pos1.0 - pos2.0) as f64;
    let dy = (pos1.1 - pos2.1) as f64;
    (dx * dx + dy * dy).sqrt()
}

pub fn apply_distance_modifier(iki_raw: f64, pos1: (i32, i32), pos2: (i32, i32)) -> f64 {
    let d = distance(pos1, pos2);
    iki_raw * (1.0 + (d * 0.05))
}
#[derive(PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
    Neutral,
}

pub fn get_hand(pos: (i32, i32)) -> Hand {
    if pos == (5, 4) {
        return Hand::Neutral;
    }
    if pos.0 <= 4 {
        Hand::Left
    } else {
        Hand::Right
    }
}

pub fn apply_hand_modifier(iki_raw: f64, prev_pos: (i32, i32), curr_pos: (i32, i32)) -> f64 {
    let h1 = get_hand(prev_pos);
    let h2 = get_hand(curr_pos);

    if h1 == Hand::Neutral || h2 == Hand::Neutral {
        return iki_raw;
    }

    if h1 == h2 {
        iki_raw * 1.15
    } else {
        iki_raw * 0.85
    }
}

pub fn shift_penalty(rng: &mut ClackRng) -> f64 {
    let sigma = SHIFT_PENALTY_SIGMA;
    let mu = SHIFT_PENALTY_MU_MS.ln() - (sigma * sigma / 2.0);
    rng.sample_log_normal(mu, sigma).clamp(SHIFT_PENALTY_MIN_MS, SHIFT_PENALTY_MAX_MS)
}
