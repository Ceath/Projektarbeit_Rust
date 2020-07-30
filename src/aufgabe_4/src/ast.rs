use std::boxed::Box;

// Type alias
pub type ExprBox = Box<dyn Expr>;

// Eq und PartialEQ erlauben das vergleichen von ExprType Werten via == oder !=
// Debug erlaubt das darstellen eines Enum-Wertes als String. z.B. ExprType::Int:= 'Int'
#[derive(Eq, PartialEq, Debug)]
pub enum ExprType {
    Int,
    Plus,
    Mul,
}

// Interface für Expr
pub trait Expr {
    fn eval(&self) -> i32;
    // vorgegeben pretty implementierung
    fn pretty(&self) -> String;
    // dient zum erkennen des Typs der Expr
    fn expr_type(&self) -> ExprType;
    // Teilaufgabe 1 Syntax. pretty mit schlauerer klammerung
    fn pretty_clever(&self) -> String;
    // wird zum cast einer trait reference zu dem eigentlichen Struct des Objekts benötigt
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct IntExpr {
    pub val: i32,
}

impl Expr for IntExpr {
    fn eval(&self) -> i32 {
        self.val
    }

    fn pretty(&self) -> String {
        self.val.to_string()
    }

    fn expr_type(&self) -> ExprType {
        ExprType::Int
    }

    // Keine klammerung vorhanden -> Identische ausgabe wie pretty()
    fn pretty_clever(&self) -> String {
        self.pretty()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl IntExpr {
    pub fn new(val: i32) -> IntExpr {
        IntExpr { val }
    }
}

// Speichert e1 und e2 als Box<dyn Expr> mit ownership der Box
pub struct PlusExpr {
    pub e1: ExprBox,
    pub e2: ExprBox,
}

impl Expr for PlusExpr {
    fn eval(&self) -> i32 {
        self.e1.eval() + self.e2.eval()
    }

    fn pretty(&self) -> String {
        format! {"({} + {})", self.e1.pretty(), self.e2.pretty()}
    }

    fn expr_type(&self) -> ExprType {
        ExprType::Plus
    }

    // Bei + wird nie eine klammerung um e1 und e2 benötigt
    // Recursive wird pretty_clever() auf e1 und e2 angewandt
    fn pretty_clever(&self) -> String {
        let e1_pretty = self.e1.pretty_clever();
        let e2_pretty = self.e2.pretty_clever();

        format!("{} + {}", e1_pretty, e2_pretty)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PlusExpr {
    // Template funktion, erlaubt ausdrücke wie PlusExpr::new(IntExpr::new(1), IntExpr::new(2)), übernimmt ownership von e1 und e2
    pub fn new<T: Expr + 'static, U: Expr + 'static>(e1: T, e2: U) -> PlusExpr {
        PlusExpr {
            e1: Box::new(e1),
            e2: Box::new(e2),
        }
    }

    // Parameter direkt als Box, übernimmt ownership
    pub fn new_box(e1: ExprBox, e2: ExprBox) -> PlusExpr {
        PlusExpr { e1, e2 }
    }
}

pub struct MulExpr {
    pub e1: ExprBox,
    pub e2: ExprBox,
}

impl Expr for MulExpr {
    fn eval(&self) -> i32 {
        self.e1.eval() * self.e2.eval()
    }

    fn pretty(&self) -> String {
        format!("({} * {})", self.e1.pretty(), self.e2.pretty())
    }

    fn expr_type(&self) -> ExprType {
        ExprType::Mul
    }

    // Ergibt 'e1.pretty_clever() * e2.pretty_clever()'
    // Sollte e1 oder e2 eine PlusExpr sein wird der spezifische ausdruck umklammert (Punkt vor Strich Regel)
    fn pretty_clever(&self) -> String {
        // Hilfsfunktion zum reduzieren von code replikationen
        // Diese Methode ist nur innerhalb von pretty_clever() aufrufbar
        fn pretty_val(e: &ExprBox) -> String {
            if let ExprType::Plus = e.expr_type() {
                format!("({})", e.pretty_clever())
            } else {
                e.pretty_clever()
            }
        }

        let e1_pretty = pretty_val(&self.e1);
        let e2_pretty = pretty_val(&self.e2);

        format!("{} * {}", e1_pretty, e2_pretty)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl MulExpr {
    pub fn new<T: Expr + 'static, U: Expr + 'static>(e1: T, e2: U) -> MulExpr {
        MulExpr {
            e1: Box::new(e1),
            e2: Box::new(e2),
        }
    }

    pub fn new_box(e1: ExprBox, e2: ExprBox) -> MulExpr {
        MulExpr { e1, e2 }
    }
}

// Unit tests. Jeder funktion von Expr(außer as_any) wurde ein eigenes Module gewidmet
// Die Beispiel sind dabei in den Modulen großenteils identisch
#[cfg(test)]
mod tests {
    use super::*;

    //expr_type()
    mod expr_type {
        use super::*;

        #[test]
        fn expr_type_int() {
            let e = IntExpr::new(0);

            assert_eq!(ExprType::Int, e.expr_type());
        }

        #[test]
        fn expr_type_plus() {
            let e = PlusExpr::new(IntExpr::new(0), IntExpr::new(1));

            assert_eq!(ExprType::Plus, e.expr_type());
        }

        #[test]
        fn expr_type_mul() {
            let e = MulExpr::new(IntExpr::new(0), IntExpr::new(1));

            assert_eq!(ExprType::Mul, e.expr_type());
        }
    }

    // eval()
    mod eval {
        use super::*;

        #[test]
        fn eval_int() {
            let e = IntExpr::new(10);

            assert_eq!(e.eval(), 10);
        }

        #[test]
        fn eval_plus() {
            // (10 + 5)
            let e = PlusExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.eval(), 15);
        }

        #[test]
        fn eval_plus_nested() {
            // ((1 + 2) + 3)
            let e = PlusExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.eval(), 6);
        }

        #[test]
        fn eval_mul() {
            // (10 * 5)
            let e = MulExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.eval(), 50);
        }

        #[test]
        fn eval_mul_nested() {
            // ((1 * 2) * 3)
            let e = MulExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.eval(), 6);
        }

        #[test]
        fn eval_nested_mul_plus() {
            // ((1 + 2) * 3)
            let e = MulExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.eval(), 9);
        }

        #[test]
        fn eval_nested_plus_plus() {
            // ((1 * 2) + 3)
            let e = PlusExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.eval(), 5);
        }
    }

    // pretty()
    mod pretty {
        use super::*;

        #[test]
        fn pretty_int() {
            let e = IntExpr::new(5);

            assert_eq!(e.pretty(), "5");
        }

        #[test]
        fn pretty_plus() {
            // (10 + 5)
            let e = PlusExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.pretty(), "(10 + 5)");
        }

        #[test]
        fn pretty_plus_nested() {
            // ((1 + 2) + 3)
            let e = PlusExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty(), "((1 + 2) + 3)");
        }

        #[test]
        fn pretty_mul() {
            // (10 * 5)
            let e = MulExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.pretty(), "(10 * 5)");
        }

        #[test]
        fn pretty_mul_nested() {
            // ((1 * 2) * 3)
            let e = MulExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty(), "((1 * 2) * 3)");
        }

        #[test]
        fn pretty_nested_mul_plus() {
            // ((1 + 2) * 3)
            let e = MulExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty(), "((1 + 2) * 3)");
        }

        #[test]
        fn pretty_nested_plus_plus() {
            // ((1 * 2) + 3)
            let e = PlusExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty(), "((1 * 2) + 3)");
        }
    }

    //pretty_clever()
    mod pretty_clever {
        use super::*;

        #[test]
        fn pretty_clever_int() {
            let e = IntExpr::new(5);

            assert_eq!(e.pretty_clever(), "5");
        }

        #[test]
        fn pretty_clever_plus() {
            // (10 + 5)
            let e = PlusExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.pretty_clever(), "10 + 5");
        }

        #[test]
        fn pretty_clever_plus_nested() {
            // ((1 + 2) + 3)
            let e = PlusExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty_clever(), "1 + 2 + 3");
        }

        #[test]
        fn pretty_clever_mul() {
            // (10 * 5)
            let e = MulExpr::new(IntExpr::new(10), IntExpr::new(5));

            assert_eq!(e.pretty_clever(), "10 * 5");
        }

        #[test]
        fn pretty_clever_mul_nested() {
            // ((1 * 2) * 3)
            let e = MulExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty_clever(), "1 * 2 * 3");
        }

        #[test]
        fn pretty_clever_nested_mul_plus() {
            // ((1 + 2) * 3)
            let e = MulExpr::new(
                PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty_clever(), "(1 + 2) * 3");
        }

        #[test]
        fn pretty_clever_nested_plus_plus() {
            // ((1 * 2) + 3)
            let e = PlusExpr::new(
                MulExpr::new(IntExpr::new(1), IntExpr::new(2)),
                IntExpr::new(3),
            );

            assert_eq!(e.pretty_clever(), "1 * 2 + 3");
        }
    }
}
