mod ast;
mod tokenizer;

use ast::{Expr, IntExpr, MulExpr, PlusExpr};
use tokenizer::{Tokenize, Tokenizer};

fn main() {
    fn print<T: Expr>(expr: &T) {
        println!("Expr '{}' to clever:= '{}', eval:= {}", expr.pretty(), expr.pretty_clever(), expr.eval());
    }

    let plus = PlusExpr::new (
        IntExpr::new( 5) ,
        PlusExpr::new(
            IntExpr { val: 10 },
            IntExpr { val: 3 })
    );
    print(&plus);

    let mul = MulExpr::new(
        IntExpr { val: 7 },
        MulExpr::new(
            plus,
            IntExpr { val: 2 })
    );
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

    let mut foo = Tokenize::new("1 + 2 (2 * 1)".to_string());
    println!("{}", foo.show());

    let mut bar = Tokenizer::new("1 + 2 (2 * 1)".to_string());
    for _i in 0..10 {
        println!("{}", bar.token);
        bar.next_token();
    }
}
