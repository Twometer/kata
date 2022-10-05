use std::{iter::Peekable, str::CharIndices};

pub struct Scanner<'a> {
    str: &'a str,
    chr: Peekable<CharIndices<'a>>,
    idx: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            chr: str.char_indices().peekable(),
            idx: 0,
        }
    }

    pub fn has_prefix(&mut self, prefix: &str) -> bool {
        self.str[self.idx..].starts_with(prefix)
    }

    pub fn has_prefix_chr(&mut self, prefix: char) -> bool {
        self.str[self.idx..].starts_with(prefix)
    }

    pub fn consume_exact(&mut self, prefix: &str) -> bool {
        if self.has_prefix(prefix) {
            for _ in 0..prefix.len() {
                if !self.advance() {
                    break;
                }
            }
            return true;
        }
        false
    }

    pub fn consume_until_any_char(&mut self, delims: &[char]) -> &'a str {
        let start = self.idx;
        while !delims.iter().any(|it| self.has_prefix_chr(*it)) {
            if !self.advance() {
                break;
            }
        }
        &self.str[start..self.idx]
    }

    pub fn consume_until_char(&mut self, delim: char) -> &'a str {
        let start = self.idx;
        while !self.has_prefix_chr(delim) {
            if !self.advance() {
                break;
            }
        }
        &self.str[start..self.idx]
    }

    pub fn consume_until_str(&mut self, delim: &str) -> &'a str {
        let start = self.idx;
        while !self.has_prefix(delim) {
            if !self.advance() {
                break;
            }
        }
        &self.str[start..self.idx]
    }

    pub fn consume_whitespace(&mut self) {
        while self.has_prefix(" ") {
            self.advance();
        }
    }

    pub fn has_remaining(&mut self) -> bool {
        self.chr.peek().is_some()
    }

    pub fn index(&mut self) -> usize {
        self.idx
    }

    fn advance(&mut self) -> bool {
        if let Some((idx, _)) = self.chr.next() {
            self.idx = idx;
            true
        } else {
            false
        }
    }
}
