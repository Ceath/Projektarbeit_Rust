use aufgabe_4::ast::{Expr, ExprBox, IntExpr, MulExpr, PlusExpr};
use aufgabe_4::parser::Parser;
use aufgabe_4::tokenizer::{Tokenize, Tokenizer};
use aufgabe_4::vm::VM;

fn main() {
    fn print<T: Expr>(expr: &T) {
        println!(
            "Expr '{}' to clever:= '{}', eval:= {}",
            expr.pretty(),
            expr.pretty_clever(),
            expr.eval()
        );
    }

    let plus = PlusExpr::new(
        IntExpr::new(5),
        PlusExpr::new(IntExpr { val: 10 }, IntExpr { val: 3 }),
    );
    print(&plus);

    let mul = MulExpr::new(IntExpr { val: 7 }, MulExpr::new(plus, IntExpr { val: 2 }));
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

    {
        let expr = Parser::new("1 + 2 * (2 * 1)".to_string()).parse().unwrap();
        let mut vm: VM = VM::from(&expr);
        println!(
            "VM of '{}' := {}. Eval(VM):= {}, eval(exp) := {}",
            expr.pretty(),
            vm.to_string(),
            vm.run().unwrap(),
            expr.eval()
        );
    }
    {
        let expr: ExprBox = Box::new(mul);
        let mut vm: VM = VM::from(&expr);
        println!(
            "VM of '{}' := {}. Eval(VM):= {}, eval(exp) := {}",
            expr.pretty(),
            vm.to_string(),
            vm.run().unwrap(),
            expr.eval()
        );
    }
}
