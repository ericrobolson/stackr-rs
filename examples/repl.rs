use stackr_rs::*;

fn main() {
    // TODO: figure out way to start repl in interpreter
    // let code = "repl \"example of repl\" print-stack";
    let mut interpreter = Interpreter::new(());
    // interpreter.evaluate(code, None).unwrap();

    // Alternative way to start repl in host
    interpreter.start_repl().unwrap();
}
