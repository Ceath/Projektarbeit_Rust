use std::string::String;
use std::boxed::Box;

pub type ExprBox =  Box<dyn Expr>;

#[derive(Eq, PartialEq, Debug)]
pub enum ExprType {
    Int,
    Plus,
    Mul,
}

pub trait Expr {
    fn eval(&self) -> i32;
    fn pretty(&self) -> String;
    fn expr_type(&self) -> ExprType;
    fn pretty_clever(&self) -> String;
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

    fn pretty_clever(&self) -> String {
        self.pretty()
    }
}

impl IntExpr {
    pub fn new(val: i32) -> IntExpr {
        IntExpr { val }
    }
}

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

    fn pretty_clever(&self) -> String {
        let e1_pretty = self.e1.pretty_clever();
        let e2_pretty = self.e2.pretty_clever();

        format!("{} + {}", e1_pretty, e2_pretty)
    }
}

impl PlusExpr {
    pub fn new<T: Expr + 'static, U: Expr + 'static>(e1: T, e2: U) -> PlusExpr {
        PlusExpr {e1: Box::new(e1), e2: Box::new(e2) }
    }

    pub fn new_box(e1: ExprBox, e2: ExprBox) -> PlusExpr {
        PlusExpr {
            e1,
            e2
        }
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

    fn pretty_clever(&self) -> String {
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
}

impl MulExpr {
    pub fn new<T: Expr + 'static, U: Expr + 'static>(e1: T, e2: U) -> MulExpr {
        MulExpr { e1: Box::new(e1), e2: Box::new(e2) }
    }

    pub fn new_box(e1: ExprBox, e2: ExprBox) -> MulExpr {
        MulExpr {
            e1,
            e2
        }
    }
}

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
            let int = IntExpr::new(10);

            assert_eq!(int.eval(), 10);
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
