use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "print-program",
        "",
        "Prints the program.",
        "",
        |interpreter| {
            let program = interpreter.stringify_program();
            println!("{}", program);
            Ok(())
        },
    );

    interpreter.register_builtin("print-stack", "", "Prints the stack.", "", |interpreter| {
        let mut buffer = String::new();
        buffer.push_str("[ ");
        for value in interpreter.stack.iter() {
            match value {
                StackValue::Address(address) => {
                    buffer.push_str(&interpreter.get_name(*address));
                }
                StackValue::Value(value) => match value {
                    Value::Number(f) => buffer.push_str(&format!("{}", f)),
                    Value::String(s) => buffer.push_str(&format!("\"{}\"", s)),
                },
            }
            buffer.push_str(" ");
        }

        buffer.push(']');

        println!("{}", buffer);

        Ok(())
    });

    interpreter.register_builtin("exit", "", "Exit the program.", "", |interpreter| {
        interpreter.exit = true;
        Ok(())
    });

    // TODO: unable to start repl from interpreter.
    // There's a bug with the program counter/evaluate code.
    // interpreter.register_builtin("repl", "", "Starts REPL mode.", "", |interpreter| {
    //     interpreter.repl_mode = true;
    //     Ok(())
    // });

    interpreter.register_builtin("repl-exit", "", "Exits REPL mode.", "", |interpreter| {
        interpreter.repl_mode = false;
        Ok(())
    });

    interpreter.register_builtin(
        "documentation",
        "",
        "Show documentation for all words",
        "",
        |interpreter| {
            interpreter.print_documentation();

            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn repl_sets_repl_mode_true() {
        // Skip because infinite loop...
        let mut interpreter = Interpreter::new(());
        register_builtins(&mut interpreter);
        interpreter.evaluate("repl", None).unwrap();
        assert!(interpreter.repl_mode);
    }

    #[test]
    fn exit_sets_should_quit_true() {
        let mut interpreter = Interpreter::new(());
        register_builtins(&mut interpreter);
        interpreter.evaluate("exit", None).unwrap();
        assert!(interpreter.exit);
    }
}
