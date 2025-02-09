use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "clear-stack",
        ".. --",
        "Clears the entire stack.",
        "clear-stack",
        |interpreter| {
            interpreter.stack.clear();

            Ok(())
        },
    );

    interpreter.register_builtin(
        "stack-size",
        "-- n",
        "Pushes the size of the stack onto the stack.",
        "stack-size",
        |interpreter| {
            let size = interpreter.stack.len() as f32;
            interpreter
                .stack
                .push(StackValue::Value(Value::Number(size)));

            Ok(())
        },
    );

    interpreter.register_builtin(
        "dup",
        "n -- n n",
        "Duplicates the top item on the stack.",
        "2 dup",
        |interpreter| {
            let top = interpreter.pop()?;
            interpreter.stack.push(top.clone());
            interpreter.stack.push(top);

            Ok(())
        },
    );

    interpreter.register_builtin(
        "swap",
        "a b -- b a",
        "Swaps the top two items on the stack.",
        "1 2 swap",
        |interpreter| {
            let a = interpreter.pop()?;
            let b = interpreter.pop()?;
            interpreter.stack.push(a);
            interpreter.stack.push(b);
            Ok(())
        },
    );

    interpreter.register_builtin(
        "drop",
        "n --",
        "Drops the top item on the stack.",
        "1 drop",
        |interpreter| {
            interpreter.pop()?;
            Ok(())
        },
    );

    interpreter.register_builtin(
        "over",
        "a b -- a b a",
        "Copies the second item on the stack to the top.",
        "1 2 over",
        |interpreter| {
            let a = interpreter.pop()?;
            let b = interpreter.pop()?;
            interpreter.stack.push(b.clone());
            interpreter.stack.push(a);
            interpreter.stack.push(b);
            Ok(())
        },
    );

    interpreter.register_builtin(
        "rot",
        "1 2 3 -- 2 3 1",
        "Rotates the top three items on the stack.",
        "1 2 3 rot",
        |interpreter| {
            let a = interpreter.pop()?;
            let b = interpreter.pop()?;
            let c = interpreter.pop()?;
            interpreter.stack.push(b);
            interpreter.stack.push(a);
            interpreter.stack.push(c);
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_stack() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1 2 3", None).unwrap();
        assert_eq!(interpreter.stack.len(), 3);
        interpreter.evaluate("clear-stack", None).unwrap();
        assert_eq!(interpreter.stack.len(), 0);
    }

    #[test]
    fn test_stack_size() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1 2 3", None).unwrap();
        assert_eq!(interpreter.stack.len(), 3);
        interpreter.evaluate("stack-size", None).unwrap();
        assert_eq!(interpreter.stack.len(), 4);
        assert_eq!(interpreter.pop_number().unwrap(), 3.0);
    }

    #[test]
    fn test_over() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 over";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.stack.len(), 3);
        assert_eq!(interpreter.pop_number().unwrap(), 1.0);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
        assert_eq!(interpreter.pop_number().unwrap(), 1.0);
    }

    #[test]
    fn test_rot() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 3 rot";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.stack.len(), 3);
        assert_eq!(interpreter.pop_number().unwrap(), 1.0);
        assert_eq!(interpreter.pop_number().unwrap(), 3.0);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
    }

    #[test]
    fn test_dup() {
        let mut interpreter = Interpreter::new(());
        let code = "2 dup";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.stack.len(), 2);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
    }

    #[test]
    fn test_swap() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 swap";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.stack.len(), 2);
        assert_eq!(interpreter.pop_number().unwrap(), 1.0);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
    }

    #[test]
    fn test_drop() {
        let mut interpreter = Interpreter::new(());
        let code = "1 drop";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.stack.len(), 0);
    }
}
