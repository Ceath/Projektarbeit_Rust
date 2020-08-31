use crate::ast::ExprBox;
use crate::parser::Parser;
use crate::vm_parser::VMParser;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use OpCode::{MUL, PLUS, PUSH};

#[derive(Debug)]
pub enum OpCode {
    // Enthält direkt den Wert von PUSH ohne eine extra Feld in Code zu benötigen
    PUSH(i32),
    PLUS,
    MUL,
}

#[derive(Debug)]
pub struct Code {
    pub kind: OpCode,
}

impl Code {
    pub fn new(kind: OpCode) -> Code {
        Code { kind }
    }

    pub fn new_val(val: i32) -> Code {
        Code { kind: PUSH(val) }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

pub fn new_push(val: i32) -> Code {
    Code::new_val(val)
}

pub fn new_plus() -> Code {
    Code::new(PLUS)
}

pub fn new_mul() -> Code {
    Code::new(MUL)
}

#[derive(Debug)]
pub struct VM {
    code: Vec<Code>,
    stack: Vec<i32>,
}

impl VM {
    pub fn new(code: Vec<Code>) -> VM {
        VM {
            code,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Option<i32> {
        self.stack = Vec::new();

        for c in &self.code {
            match c.kind {
                PUSH(val) => self.stack.push(val),
                MUL => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    if let (Some(r), Some(l)) = (right, left) {
                        self.stack.push(r * l);
                    }
                }
                PLUS => {
                    let right = self.stack.pop();
                    let left = self.stack.pop();
                    if let (Some(r), Some(l)) = (right, left) {
                        self.stack.push(r + l);
                    }
                }
            }
        }

        match self.stack.last() {
            None => None,
            Some(v) => Some(*v),
        }
    }
}

// Erlaubt das direkt Konvertieren eines &str zu einer VM
// Verwendet dafür Parser zum Konvertieren auf Expr und anschließend vm_parser zum parsen von Expr zu VM
impl TryFrom<&str> for VM {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let expr = Parser::new(value.to_string()).parse();
        match expr {
            None => Err(()),
            Some(e) => Ok(Self::from(&e)),
        }
    }
}

// Erlaubt das direkt parsen einer &ExprBox zu einer VM mithilfe von vm_parser
impl From<&ExprBox> for VM {
    fn from(e: &ExprBox) -> Self {
        VMParser::new().parse(e)
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VM[ ")?;
        for c in &self.code {
            write!(f, "{}, ", c)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

// Tests aus testVM mit zusätzlichen Tests
// Unit-test. Enthält Ausgaben zum Betrachten der Ausführung
#[cfg(test)]
mod tests {
    use super::*;

    fn show_vm_res(r: &Option<i32>) {
        print!("VM stack (top): ");
        match r {
            None => println!("{}", "empty"),
            Some(t) => println!("{}", t),
        }
    }

    #[test]
    fn test_vm_1() {
        let vc = vec![new_push(1), new_push(2), new_push(3), new_mul(), new_plus()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(Some(7), res);
    }

    #[test]
    fn test_vm_2() {
        let vc = vec![new_push(2), new_push(3), new_push(5), new_plus(), new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(Some(16), res);
    }

    #[test]
    fn test_vm_3() {
        let vc = vec![];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_4() {
        let vc = vec![new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_5() {
        let vc = vec![new_plus()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_6() {
        let vc = vec![new_push(1), new_mul()];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(None, res);
    }

    #[test]
    fn test_vm_7() {
        let vc = vec![new_push(1)];
        let res = VM::new(vc).run();

        show_vm_res(&res);
        assert_eq!(Some(1), res);
    }
}
