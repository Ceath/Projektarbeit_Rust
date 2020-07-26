mod ast;

use ast::{Expr, IntExpr, MulExpr, PlusExpr};

fn main() {
    fn print<T: Expr>(expr: &T) {
        println!("Expr '{}' to clever:= '{}', eval:= {}", expr.pretty(), expr.pretty_clever(), expr.eval());
    }

    let plus = PlusExpr {
        e1: IntExpr { val: 5 },
        e2: PlusExpr {
            e1: IntExpr { val: 10 },
            e2: IntExpr { val: 3 },
        },
    };
    print(&plus);

    let mul = MulExpr {
        e1: IntExpr { val: 7 },
        e2: MulExpr {
            e1: plus,
            e2: IntExpr { val: 2 },
        },
    };
    print(&mul);
    println!();

    let ex1 = PlusExpr::new(
        IntExpr::new(1),
        MulExpr::new(IntExpr::new(3), IntExpr::new(5)),
    );
    print(&ex1);

    let ex2 = MulExpr::new(
        IntExpr::new(1),
        PlusExpr::new(IntExpr::new(3), IntExpr::new(5)),
    );
    print(&ex2);
    eprintln!();

    let ex_eq1 = PlusExpr::new(
        IntExpr::new(1),
        PlusExpr::new(IntExpr::new(2), IntExpr::new(3)),
    );
    let ex_eq2 = PlusExpr::new(
        PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
        IntExpr::new(3),
    );

    print(&ex_eq1);
    print(&ex_eq2);
    println!(
        "ex_eq1.pretty_clever() == ex_eq2.pretty_clever():= {}",
        ex_eq1.pretty_clever() == ex_eq2.pretty_clever()
    );
}
