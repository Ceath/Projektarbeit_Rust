mod ast;

use ast::{IntExpr, PlusExpr, MulExpr, Expr, ExprType};

fn main() {
    println!("Hello, World");
    let plus = PlusExpr {
        e1: IntExpr {
            val: 5
        },
        e2: PlusExpr {
            e1: IntExpr { val: 10 },
            e2: IntExpr { val: 3 },
        },
    };

    println!("{} = {}", plus.pretty(), plus.eval());

    let mul = MulExpr {
        e1: IntExpr { val: 7 },
        e2: MulExpr {
            e1: plus,
            e2: IntExpr { val: 2 },
        },
    };

    println!("{} = {}", mul.pretty(), mul.eval());
}
