use stacker_rs::*;

fn main() {
    let code = "1 1 +";
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None);

    println!("1 1 + ={}", interpreter.pop_number().unwrap());
}
