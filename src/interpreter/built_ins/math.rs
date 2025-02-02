use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "+",
        "n n -- n",
        "Add two numbers.",
        "1 2 +",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            interpreter.push_number(a + b);
            Ok(())
        },
    );

    interpreter.register_builtin(
        "-",
        "n n -- n",
        "Subtract two numbers.",
        "5 3 -",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            interpreter.push_number(b - a); // Note: b - a since stack order
            Ok(())
        },
    );

    interpreter.register_builtin(
        "*",
        "n n -- n",
        "Multiply two numbers.",
        "4 2 *",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            interpreter.push_number(a * b);
            Ok(())
        },
    );

    interpreter.register_builtin(
        "/",
        "n n -- n",
        "Divide two numbers.",
        "2 6 /",
        |interpreter| {
            let b = interpreter.pop_number()?;
            let a = interpreter.pop_number()?;
            if a == 0.0 {
                return Err(("Division by zero".to_string(), interpreter.get_location()));
            }
            interpreter.push_number(b / a); // Note: b / a since stack order
            Ok(())
        },
    );

    interpreter.register_builtin(
        "%",
        "n n -- n",
        "Modulo two numbers.",
        "2 10 %",
        |interpreter| {
            let b = interpreter.pop_number()?;
            let a = interpreter.pop_number()?;
            interpreter.push_number(b % a);
            Ok(())
        },
    );

    interpreter.register_builtin(
        "int",
        "n -- n",
        "Truncates a number to an integer.",
        "1.3 int",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let int = a as i32;
            interpreter.push_number(int as f32);
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("2.6 int 1.3 int", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 1.0);
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
    }

    #[test]
    fn modulo() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("2 10 %", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 0.0);
    }

    #[test]
    fn test_add() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1 2 +", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 3.0);
    }

    #[test]
    fn test_subtract() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("5 3 -", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 2.0);
    }

    #[test]
    fn test_multiply() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("4 2 *", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 8.0);
    }

    #[test]
    fn test_divide() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("2 6 /", None).unwrap();
        assert_eq!(interpreter.pop_number().unwrap(), 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let mut interpreter = Interpreter::new(());
        let result = interpreter.evaluate("0 1 /", None);
        assert!(result.is_err());
        if let Err((msg, _)) = result {
            assert_eq!(msg, "Division by zero");
        }
    }
}
