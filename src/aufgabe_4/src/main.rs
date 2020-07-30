use aufgabe_4::ast::{Expr, ExprBox, IntExpr, MulExpr, PlusExpr};
use aufgabe_4::parser::Parser;
use aufgabe_4::tokenizer::Tokenize;
use aufgabe_4::vm::VM;

fn main() {
    expr_examples();
    tokenize_examples();
    vm_parser_examples();
}

fn expr_examples() {
    fn print<T: Expr>(expr: &T) {
        println!(
            "Expr '{}' to clever:= '{}', eval:= {}",
            expr.pretty(),
            expr.pretty_clever(),
            expr.eval()
        );
    }

    println!("Expr Examples:");
    // (5 + (10 + 3))
    // Verwendet den Template Konstruktor
    let plus = PlusExpr::new(
        IntExpr::new(5),
        PlusExpr::new(IntExpr { val: 10 }, IntExpr { val: 3 }),
    );
    print(&plus);

    // (7 * ((5 + (10 + 3)) * 2))
    let mul = MulExpr::new(IntExpr { val: 7 }, MulExpr::new(plus, IntExpr { val: 2 }));
    print(&mul);
    println!();

    // (1 + (3 * 5))
    let ex1 = PlusExpr::new(
        IntExpr::new(1),
        MulExpr::new(IntExpr::new(3), IntExpr::new(5)),
    );
    print(&ex1);

    // (1 * (3 + 5))
    let ex2 = MulExpr::new(
        IntExpr::new(1),
        PlusExpr::new(IntExpr::new(3), IntExpr::new(5)),
    );
    print(&ex2);
    eprintln!();

    // (1 + (2 + 3))
    let ex_eq1 = PlusExpr::new(
        IntExpr::new(1),
        PlusExpr::new(IntExpr::new(2), IntExpr::new(3)),
    );
    // ((1 + 2) + 3)
    let ex_eq2 = PlusExpr::new(
        PlusExpr::new(IntExpr::new(1), IntExpr::new(2)),
        IntExpr::new(3),
    );

    print(&ex_eq1);
    print(&ex_eq2);
    // Beide pretty_clever() sollten die gleiche Ausgabe haben da ex_eq1 und ex_eq2 grammatisch gleich sind
    println!(
        "ex_eq1.pretty_clever() == ex_eq2.pretty_clever():= {}",
        ex_eq1.pretty_clever() == ex_eq2.pretty_clever()
    );
    println!("\n");
}

fn tokenize_examples() {
    println!("Tokenize Examples:");
    {
        let s = "(2 + 1) * 0";
        let mut t = Tokenize::new(s.to_string());
        println!("Tokenize of '{}':= [{}]", &s, t.show());
    }
    {
        let s = "(2 * (5 + 101)) * (7 + 0)";
        let mut t = Tokenize::new(s.to_string());
        println!("Tokenize of '{}':= [{}]", &s, t.show());
    }
    println!("\n");
}

fn vm_parser_examples() {
    println!("VMParser Examples:");
    {
        // (1 + 2 * (2 * 1))
        let expr = Parser::new("1 + 2 * (2 * 1)".to_string()).parse().unwrap();
        let mut vm: VM = VM::from(&expr);
        println!(
            "VM of '{}':= {}. Eval(VM):= {}, eval(exp):= {}",
            expr.pretty(),
            vm.to_string(),
            vm.run().unwrap(),
            expr.eval()
        );
    }
    {
        // ((2 * (5 + 101)) * (7 + 0))
        let expr = Parser::new("(2 * (5 + 101)) * (7 + 0)".to_string())
            .parse()
            .unwrap();
        let mut vm: VM = VM::from(&expr);
        println!(
            "VM of '{}':= {}. Eval(VM):= {}, eval(exp):= {}",
            expr.pretty(),
            vm.to_string(),
            vm.run().unwrap(),
            expr.eval()
        );
    }
    {
        let expr: ExprBox = Box::new(MulExpr::new(
            IntExpr::new(7),
            MulExpr::new(
                PlusExpr::new(
                    IntExpr::new(5),
                    PlusExpr::new(IntExpr::new(10), IntExpr::new(3)),
                ),
                IntExpr::new(2),
            ),
        ));
        let mut vm: VM = VM::from(&expr);
        println!(
            "VM of '{}':= {}. Eval(VM):= {}, eval(exp):= {}",
            expr.pretty(),
            vm.to_string(),
            vm.run().unwrap(),
            expr.eval()
        );
    }
}
