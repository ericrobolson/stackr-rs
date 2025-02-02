use stackr_rs::*;

fn main() {
    let code = r#"

    0
    begin
        print-stack
    
        dup 10 <=
        if
            drop
            "loop finished"
            print-stack
            break
        end
        1 + 
    loop
    
    "#;
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None);
}
