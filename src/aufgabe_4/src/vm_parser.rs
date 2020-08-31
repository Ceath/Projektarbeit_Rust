use crate::ast::{Expr, ExprBox, IntExpr, MulExpr, PlusExpr};
use crate::vm::{new_mul, new_plus, new_push, Code, VM};

// Teilaufgabe 2 Semantik
pub struct VMParser {
    code: Vec<Code>,
}

// Konvertiert eine Expr in eine VM
// Arbeitet sich rekursiv durch die eingabe Expr durch und wandelt die einzelnen teile dabei in Code um und hängt diese self.code an
impl VMParser {
    // IntExpr werden direkt als PUSH mit ihren Wert zurückgegeben
    fn parse_int(&mut self, e: &IntExpr) -> Code {
        new_push(e.eval())
    }

    // Parse einer MulExpr
    // hängt zuerst den Code aus e1 und anschließend den aus e2 an self.code an bevor MUL zurückgegeben wird
    fn parse_mul(&mut self, e: &MulExpr) -> Code {
        let e1_code = self.parse_t(&e.e1);
        self.code.push(e1_code);
        let e2_code = self.parse_t(&e.e2);
        self.code.push(e2_code);
        new_mul()
    }

    // Parse eine PlusExpr
    // hängt zuerst den Code aus e1 und anschließend den aus e2 an self.code an bevor PLUS zurückgegeben wird
    fn parse_plus(&mut self, e: &PlusExpr) -> Code {
        let e_as_plus = e.as_any().downcast_ref::<PlusExpr>().unwrap();

        let e1_code = self.parse_t(&e_as_plus.e1);
        self.code.push(e1_code);
        let e2_code = self.parse_t(&e_as_plus.e2);
        self.code.push(e2_code);
        new_plus()
    }

    // Funktion zum Parsen einer allgemeinen Expr
    fn parse_t(&mut self, e: &ExprBox) -> Code {
        // Mithilfe von Any ist das cast eines Traits zum ursprungs Struct möglich
        // Dies ist notwendig um auf e1 und e2 von MulExpr und PlusExpr zuzugreifen
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

    // Parse einer Expr zu einer VM, die Expr wird dabei nicht(!) konsumiert und diese funktion darf mehrmals ausgeführt werden
    pub fn parse(&mut self, e: &ExprBox) -> VM {
        let code = self.parse_t(e);
        self.code.push(code);
        // Direktes bewegen eines Feldes ist nicht erlaubt, da das Feld immer einen Wert besitzen muss
        // replace dient hierbei als swap für self.code mit einem neuen Vec
        let r = std::mem::replace(&mut self.code, Vec::new());
        // Die neue VM ergreift besitzt des vorherigen self.code, dadurch wird unnötiges kopieren vermieden
        VM::new(r)
    }

    // Statische Methode für parse
    pub fn parse_static(e: &ExprBox) -> VM {
        let mut parser = VMParser::new();
        parser.parse(e)
    }

    pub fn new() -> VMParser {
        VMParser { code: Vec::new() }
    }
}

// Unit-Tests. Die Beispiel sind größtenteils aus ast entnommen
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_parse_int() {
        let e: ExprBox = Box::new(IntExpr::new(10));
        let mut vm = VMParser::parse_static(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_plus_nested() {
        // ((1 + 2) + 3)
        let e: ExprBox = Box::new(PlusExpr::new(
            PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::parse_static(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_mul_nested() {
        // ((1 * 2) * 3)
        let e: ExprBox = Box::new(MulExpr::new(
            MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::parse_static(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_nested_mul_plus() {
        // ((1 + 2) * 3)
        let e: ExprBox = Box::new(MulExpr::new(
            PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::parse_static(&e);

        assert_eq!(Some(e.eval()), vm.run());
    }

    #[test]
    fn vm_parse_nested_plus_plus() {
        // ((1 * 2) + 3)
        let e: ExprBox = Box::new(PlusExpr::new(
            MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
            IntExpr::new(3),
        ));
        let mut vm = VMParser::parse_static(&e);

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
        let mut vm = VMParser::parse_static(&e);
        let res = vm.run();

        assert_eq!(Some(e.eval()), res);
        assert_eq!(Some(5), res);
    }
}
