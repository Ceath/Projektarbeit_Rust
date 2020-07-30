use crate::tokenizer::TokenT::NUM;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Eq, PartialEq)]
pub enum TokenT {
    EOS,
    // Tokenizer wurde erweiterter um alle (positiven) numerischen werte zu erkennen, nicht nur 0, 1 und 2
    NUM(i32),
    OPEN,
    CLOSE,
    PLUS,
    MUL,
}

// Darstellung eines TokenT wertes in format mithilfe der default implementierung des Debug traits
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
        // EOS mus angehängt werden da es in v nicht vorkommt (siehe next())
        s.push_str(&TokenT::EOS.to_string());
        s
    }
}

// Da next() aus dem Original Code ähnlichkeit mit einem Iterator aufweist wurde dieser hier implementiert.
// Dies erlaubt das aufrufen von Funktionen wie collect() ohne diese manuell zu implementieren.
// Aufgrund dessen terminiert diese Funktion nicht mit einem EOS sondern mit einem None, dies wird in Tokenizer behandelt
impl Iterator for Tokenize {
    type Item = TokenT;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut c = self.s.chars().nth(self.pos);

            // Prüft ob es sich bei c um eine Zahl handelt, wenn ja wird die komplette Zahl erkannt und als NUM zurückgegeben
            let mut num_val: Option<i32> = None;
            while let Some(Some(num)) = c.map(|ch| ch.to_digit(10)) {
                num_val = Some(num_val.unwrap_or(0) * 10 + num as i32);

                self.pos += 1;
                c = self.s.chars().nth(self.pos);
            }
            if let Some(n) = num_val {
                return Some(NUM(n));
            }

            self.pos += 1;
            match c {
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

    // Ist tokenize weiter als der letzte char des strings gibt next() ein None zurück, dies wird zu einem EOS umgewandelt
    pub fn next_token(&mut self) {
        self.token = self.tokenize.next().unwrap_or(TokenT::EOS);
    }
}

// Rust erlaubt nicht das vererben von Structs und die verwendung eines Trait ist aufwändig da jede funktion extra manuell implementiert werden müsste
// Deref ermöglicht das aufrufen aller funktionen von Tokenize
// Deref ist vergleichbar mit dem Überschreiben des ->(Pointer) Operator in C++
impl Deref for Tokenizer {
    type Target = Tokenize;

    fn deref(&self) -> &Self::Target {
        &self.tokenize
    }
}
impl DerefMut for Tokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokenize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::TokenT::{CLOSE, EOS, MUL, OPEN, PLUS};

    fn print_tokenize(input: &str) -> String {
        let mut t = Tokenize::new(input.to_string());
        let s = t.show();
        println!("Tokenize of '{}':= [{}]", input, s);
        s
    }

    #[test]
    fn tokenize_1() {
        let token_string = print_tokenize("1 + 0 * 2");
        assert_eq!(token_string, "NUM(1);PLUS;NUM(0);MUL;NUM(2);EOS")
    }

    #[test]
    fn tokenize_2() {
        let token_string = print_tokenize("(1 + 2 ) * 0 ");
        assert_eq!(token_string, "OPEN;NUM(1);PLUS;NUM(2);CLOSE;MUL;NUM(0);EOS")
    }

    #[test]
    fn tokenize_3() {
        let token_string = print_tokenize("1 + (20 + 131)");
        assert_eq!(
            token_string,
            "NUM(1);PLUS;OPEN;NUM(20);PLUS;NUM(131);CLOSE;EOS"
        )
    }

    #[test]
    fn tokenize_4() {
        let token_string = print_tokenize("1 +/ 0 * 2 Foo");
        assert_eq!(token_string, "NUM(1);PLUS;NUM(0);MUL;NUM(2);EOS")
    }

    #[test]
    fn tokenizer_1() {
        let mut t = Tokenizer::new("1 + 0 * 2".to_string());
        let v = vec![NUM(1), PLUS, NUM(0), MUL, NUM(2), EOS, EOS, EOS];

        for val in v {
            assert_eq!(val, t.token);
            t.next_token();
        }
    }

    #[test]
    fn tokenizer_2() {
        let mut t = Tokenizer::new("(1 + 0) * (2)".to_string());
        let v = vec![
            OPEN,
            NUM(1),
            PLUS,
            NUM(0),
            CLOSE,
            MUL,
            OPEN,
            NUM(2),
            CLOSE,
            EOS,
            EOS,
            EOS,
        ];

        for val in v {
            assert_eq!(val, t.token);
            t.next_token();
        }
    }
}
