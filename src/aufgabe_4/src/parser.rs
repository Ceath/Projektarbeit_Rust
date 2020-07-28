use crate::ast::{ExprBox, IntExpr, MulExpr, PlusExpr};
use crate::tokenizer::{TokenT, Tokenizer};

struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn parse_e(&mut self) -> Option<ExprBox> {
        match self.parse_t() {
            None => None,
            Some(e) => self.parse_e2(e),
        }
    }

    fn parse_e2(&mut self, left: ExprBox) -> Option<ExprBox> {
        if self.tokenizer.token == TokenT::PLUS {
            self.tokenizer.next_token();

            let right = self.parse_t();
            return match right {
                None => right,
                Some(r) => self.parse_e2(Box::new(PlusExpr::new_box(left, r))),
            };
        }
        Some(left)
    }

    fn parse_t(&mut self) -> Option<ExprBox> {
        let f = self.parse_f();
        match f {
            None => f,
            Some(e) => self.parse_t2(e),
        }
    }

    fn parse_t2(&mut self, left: ExprBox) -> Option<ExprBox> {
        if self.tokenizer.token == TokenT::MUL {
            self.tokenizer.next_token();

            let right = self.parse_f();
            return match right {
                None => right,
                Some(r) => self.parse_t2(Box::new(MulExpr::new_box(left, r))),
            };
        }
        Some(left)
    }

    fn parse_f(&mut self) -> Option<ExprBox> {
        fn int_exp(sel: &mut Parser, val: i32) -> Option<ExprBox> {
            sel.tokenizer.next_token();
            return Some(Box::new(IntExpr::new(val)));
        };

        return match self.tokenizer.token {
            TokenT::ZERO => int_exp(self, 0),
            TokenT::ONE => int_exp(self, 1),
            TokenT::TWO => int_exp(self, 2),
            TokenT::OPEN => {
                self.tokenizer.next_token();
                let e = self.parse_e();
                if e.is_none() || self.tokenizer.token != TokenT::CLOSE {
                    return e;
                }
                self.tokenizer.next_token();
                e
            }
            _ => None,
        };
    }

    pub fn new(s: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(s),
        }
    }

    pub fn parse(&mut self) -> Option<ExprBox> {
        self.parse_e()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn display_full(s: &str) -> String {
        let mut p = Parser::new(s.to_string());
        let expr = p.parse();
        let pretty = if let Some(e) = expr {
            e.pretty_clever()
        } else {
            "nothing".to_string()
        };

        println!("Parsed '{}' to '{}'", s, &pretty);
        pretty
    }

    #[test]
    fn test_parse_1() {
        assert_eq!("1", display_full("1"));
    }

    #[test]
    fn test_parse_2() {
        assert_eq!("1 + 0", display_full("1 + 0 "));
    }

    #[test]
    fn test_parse_3() {
        assert_eq!("1 + 0", display_full("1 + (0) "));
    }

    #[test]
    fn test_parse_4() {
        assert_eq!("1 + 2 * 0", display_full("1 + 2 * 0 "));
    }

    #[test]
    fn test_parse_5() {
        assert_eq!("1 * 2 + 0", display_full("1 * 2 + 0 "));
    }

    #[test]
    fn test_parse_6() {
        assert_eq!("(1 + 2) * 0", display_full("(1 + 2) * 0 "));
    }

    #[test]
    fn test_parse_7() {
        assert_eq!("(1 + 2) * 0 + 2", display_full("(1 + 2) * 0 + 2"));
    }

    #[test]
    fn test_parse_8() {
        assert_eq!("nothing", display_full("Foo") );
    }

    #[test]
    fn test_parse_9() {
        assert_eq!("nothing", display_full(""));
    }
}
