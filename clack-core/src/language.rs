use crate::constants::{COMMON_WORD_MULTIPLIER, DIFFICULT_WORD_MIN_LENGTH, DIFFICULT_WORD_MULTIPLIER};

pub fn is_common_word(word: &str) -> bool {
    let w = word.to_lowercase();
    matches!(w.as_str(),
        "the" | "be" | "to" | "of" | "and" | "a" | "in" | "that" | "have" | "i" |
        "it" | "for" | "not" | "on" | "with" | "he" | "as" | "you" | "do" | "at" |
        "this" | "but" | "his" | "by" | "from" | "they" | "we" | "say" | "her" | "she" |
        "or" | "an" | "will" | "my" | "one" | "all" | "would" | "there" | "their" | "what" |
        "so" | "up" | "out" | "if" | "about" | "who" | "get" | "which" | "go" | "me" |
        "when" | "make" | "can" | "like" | "time" | "no" | "just" | "him" | "know" | "take" |
        "people" | "into" | "year" | "your" | "good" | "some" | "could" | "them" | "see" | "other" |
        "than" | "then" | "now" | "look" | "only" | "come" | "its" | "over" | "think" | "also" |
        "back" | "after" | "use" | "two" | "how" | "our" | "work" | "first" | "well" | "way" |
        "even" | "new" | "want" | "because" | "any" | "these" | "give" | "day" | "most" | "us"
    )
}

pub fn apply_word_modifier(iki_raw: f64, word: &str) -> f64 {
    if is_common_word(word) {
        iki_raw * COMMON_WORD_MULTIPLIER
    } else if word.chars().count() >= DIFFICULT_WORD_MIN_LENGTH {
        iki_raw * DIFFICULT_WORD_MULTIPLIER
    } else {
        iki_raw
    }
}
