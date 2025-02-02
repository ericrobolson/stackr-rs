use stackr_rs::*;

fn main() {
    let code = r#"
    : squared
        "n -- n^2"
        "Squares the top of the stack"
        "2 squared"
        dup *
    ;

    2 squared
    print-stack
    "#;
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None).unwrap();
}
