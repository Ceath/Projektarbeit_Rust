use std::string::String;


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
    pub val: i32
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

    fn pretty_clever(&self) -> String {self.pretty()}

}

impl IntExpr {
    pub fn new(val: i32) -> IntExpr {
        IntExpr {
            val
        }
    }
}


pub struct PlusExpr<T: Expr, U: Expr> {
    pub e1: T,
    pub e2: U,
}

impl<T: Expr, U: Expr> Expr for PlusExpr<T, U> {
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
        fn pretty_val<E: Expr>(e: &E) -> String {
            if let ExprType::Mul = e.expr_type() {
                format!("({})",e.pretty_clever())
            }
            else { e.pretty_clever()}
        }

        let e1_pretty = pretty_val(&self.e1);
        let e2_pretty = pretty_val(&self.e2);

        format!("{} + {}", e1_pretty, e2_pretty)
    }
}

impl<T: Expr, U: Expr> PlusExpr<T, U> {
    pub fn new(e1: T, e2: U) -> PlusExpr<T,U> {
        PlusExpr {
            e1,
            e2
        }
    }
}


pub struct MulExpr<T: Expr, U: Expr> {
    pub e1: T,
    pub e2: U,
}

impl<T: Expr, U: Expr> Expr for MulExpr<T, U> {
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
        fn pretty_val<E: Expr>(e: &E) -> String {
            if let ExprType::Plus = e.expr_type() {
                format!("({})", e.pretty_clever())}
            else {e.pretty_clever()}
        }

        let e1_pretty = pretty_val(&self.e1);
        let e2_pretty = pretty_val(&self.e2);

        format!("{} * {}", e1_pretty, e2_pretty)
    }
}

impl<T: Expr, U: Expr> MulExpr<T,U> {
    pub fn new(e1: T, e2: U) -> MulExpr<T,U> {
        MulExpr {
            e1,
            e2
        }
    }
}
