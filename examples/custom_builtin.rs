use stackr_rs::*;

fn main() {
    let state: u32 = 0;
    let mut interpreter = Interpreter::new(state);

    interpreter.register_builtin(
        "increment-state",
        "--",
        "Increments the state.",
        "increment-state",
        |interpreter| {
            interpreter.state += 1;
            Ok(())
        },
    );

    interpreter.register_builtin(
        "get-state",
        "-- n",
        "Gets the state.",
        "get-state",
        |interpreter| {
            interpreter.push_number(interpreter.state as f32);
            Ok(())
        },
    );

    let code = r#"
    print-stack
    increment-state
    get-state
    "The state has been modified!"
    print-stack
    "#;

    println!("State before execution: {:?}", interpreter.state);
    interpreter.evaluate(code, None).unwrap();
    println!("State after execution: {:?}", interpreter.state);
}
