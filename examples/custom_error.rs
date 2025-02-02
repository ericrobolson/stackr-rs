use stackr_rs::*;

fn main() {
    let state: u32 = 0;
    let mut interpreter = Interpreter::new(state);

    interpreter.register_builtin(
        "example-error",
        "-- n",
        "returns an error",
        "get-state",
        |interpreter| {
            //
            Err(("This is an error".to_string(), interpreter.location()))
        },
    );

    let code = r#"
    example-error
    "#;

    let result = interpreter.evaluate(code, None);
    println!("Result: {:?}", result);
}
