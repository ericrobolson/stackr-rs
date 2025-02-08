use stackr_rs::*;

fn main() {
    let code = r#"
    1 2 [ + ] @
    "#;
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None).unwrap();

    println!(
        "1 2 [ + ] @  should return {}",
        interpreter.pop_number().unwrap()
    );
}
