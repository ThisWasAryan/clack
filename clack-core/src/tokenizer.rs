pub struct Tokenizer {
    pub buffer: String,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn feed(&mut self, data: &[u8]) {
        if let Ok(s) = std::str::from_utf8(data) {
            self.buffer.push_str(s);
        }
    }

    pub fn has_word_ready(&self) -> bool {
        self.buffer.contains(' ') || self.buffer.contains('\n') || self.buffer.contains('\t')
    }

    pub fn is_sentence_boundary(word: &str) -> bool {
        word.ends_with('.') || word.ends_with('!') || word.ends_with('?')
    }
}
