use crate::ast::{Expr, ExprBox, IntExpr, MulExpr, PlusExpr};
use crate::vm::{new_mul, new_plus, new_push, Code, VM};

pub struct VMParser {
    code: Vec<Code>,
}

impl VMParser {
    fn parse_int(&mut self, e: &IntExpr) -> Code {
        new_push(e.eval())
    }

    fn parse_mul(&mut self, e: &MulExpr) -> Code {
        let e1_code = self.parse_t(&e.e1);
        self.code.push(e1_code);
        let e2_code = self.parse_t(&e.e2);
        self.code.push(e2_code);
        new_mul()
    }

    fn parse_plus(&mut self, e: &PlusExpr) -> Code {
        let e_as_plus = e.as_any().downcast_ref::<PlusExpr>().unwrap();

        let e1_code = self.parse_t(&e_as_plus.e1);
        self.code.push(e1_code);
        let e2_code = self.parse_t(&e_as_plus.e2);
        self.code.push(e2_code);
        new_plus()
    }

    fn parse_t(&mut self, e: &ExprBox) -> Code {
        if let Some(r) = e.as_any().downcast_ref::<IntExpr>() {
            self.parse_int(r)
        } else if let Some(r) = e.as_any().downcast_ref::<MulExpr>() {
            self.parse_mul(r)
        } else if let Some(r) = e.as_any().downcast_ref::<PlusExpr>() {
            self.parse_plus(r)
        } else {
            panic!("Could not parse Expr to VM. Unexpected Expr type!")
        }
    }

    pub fn parse(&mut self, e: &ExprBox) -> VM {
        let code = self.parse_t(e);
        self.code.push(code);
        let r = std::mem::replace(&mut self.code, Vec::new());
        VM::new(r)
    }

    pub fn new() -> VMParser {
        VMParser { code: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_parse_int() {
        let e: ExprBox = Box::new(IntExpr::new(10));
        let mut vm = VMParser::new().parse(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_plus_nested() {
        // ((1 + 2) + 3)
        let e: ExprBox = Box::new(PlusExpr::new(
            PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::new().parse(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_mul_nested() {
        // ((1 * 2) * 3)
        let e: ExprBox = Box::new(MulExpr::new(
            MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::new().parse(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_nested_mul_plus() {
        // ((1 + 2) * 3)
        let e: ExprBox = Box::new(MulExpr::new(
            PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::new().parse(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_nested_plus_plus() {
        // ((1 * 2) + 3)
        let e: ExprBox = Box::new(PlusExpr::new(
            MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::new().parse(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_complex() {
        // ((1 + (2 * (2 * 1)))
        let e: ExprBox = Box::new(PlusExpr::new(
            IntExpr::new(1),
            MulExpr::new(
                IntExpr::new(2),
                MulExpr::new(IntExpr::new(2), IntExpr::new(1)),
            ),
        ));
        let mut vm = VMParser::new().parse(&e);
        println!("{}", &vm);

        assert_eq!(Some(e.eval()), vm.run());
    }
}
