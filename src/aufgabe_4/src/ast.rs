use std::string::String;


pub enum ExprType {
    Int,
    Plus,
    Mul,
}

pub trait Expr {
    fn eval(&self) -> i32;
    fn pretty(&self) -> String;
    fn expr_type() -> ExprType;
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

    fn expr_type() -> ExprType {
        ExprType::Int
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
        let x = PlusExpr { e1: IntExpr { val: 3 }, e2: IntExpr { val: 2 } };

        format! {"({} + {})", self.e1.pretty(), self.e2.pretty()}
    }

    fn expr_type() -> ExprType {
        ExprType::Plus
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

    fn expr_type() -> ExprType {
        ExprType::Mul
    }
}
