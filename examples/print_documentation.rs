use stacker_rs::*;

fn main() {
    println!("This is how you print documentation");
    let code = r#"
    documentation
    "#;
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None).unwrap();
}
