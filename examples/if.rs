use stackr_rs::*;

fn main() {
    let code = r#"
    "Example of if"
    print-stack
    drop

    1 if 
        "evaluated when true!"
        print-stack
        drop
    end

    "Now we do an example of an else"
    print-stack
    drop

    0 if 
        "Not ran"
        print-stack
        drop
    else
        "else is ran"
        print-stack
        drop
    end
    "#;
    let mut interpreter = Interpreter::new(());
    interpreter.evaluate(code, None).unwrap();
}
