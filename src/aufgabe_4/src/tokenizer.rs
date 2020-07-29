use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub enum TokenT {
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MUL,
}

impl Display for TokenT {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

pub struct Tokenize {
    s: String,
    pos: usize,
}

impl Tokenize {
    pub fn new(s: String) -> Tokenize {
        Tokenize { s, pos: 0 }
    }

    pub fn show(&mut self) -> String {
        let v: Vec<TokenT> = self.by_ref().collect();
        let mut s = String::new();

        for t in v {
            s.push_str(&t.to_string());
            s.push(';');
        }
        s
    }
}

impl Iterator for Tokenize {
    type Item = TokenT;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.s.len() <= self.pos {
                return None;
            }

            let c = self.s.chars().nth(self.pos);
            self.pos += 1;
            match c {
                Some('0') => return Some(TokenT::ZERO),
                Some('1') => return Some(TokenT::ONE),
                Some('2') => return Some(TokenT::TWO),
                Some('(') => return Some(TokenT::OPEN),
                Some(')') => return Some(TokenT::CLOSE),
                Some('+') => return Some(TokenT::PLUS),
                Some('*') => return Some(TokenT::MUL),
                None => return None,
                _ => (),
            }
        }
    }
}

pub struct Tokenizer {
    pub token: TokenT,
    tokenize: Tokenize,
}

impl Tokenizer {
    pub fn new(s: String) -> Tokenizer {
        let mut tokenize = Tokenize::new(s);
        let token = tokenize.next().unwrap_or(TokenT::EOS);
        Tokenizer { tokenize, token }
    }

    pub fn next_token(&mut self) {
        self.token = self.tokenize.next().unwrap_or(TokenT::EOS);
    }
}

impl Deref for Tokenizer {
    type Target = Tokenize;

    fn deref(&self) -> &Self::Target {
        &self.tokenize
    }
}
