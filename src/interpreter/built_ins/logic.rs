use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "&&",
        "n n -- 0|1",
        "Checks if both numbers are true. Puts 1 on the stack if they are, 0 otherwise.",
        "1 1 &&",
        |interpreter| {
            let a = interpreter.pop_bool()?;
            let b = interpreter.pop_bool()?;
            if a && b {
                interpreter.push_number(0.0);
            } else {
                interpreter.push_number(1.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        "||",
        "n n -- 0|1",
        "Checks if one of the numbers is true. Puts 1 on the stack if they are, 0 otherwise.",
        "0 1 ||",
        |interpreter| {
            let a = interpreter.pop_bool()?;
            let b = interpreter.pop_bool()?;
            if a || b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        "!",
        "n -- 0|1",
        "Inverts the boolean value on the stack. Puts 1 on the stack if the value is 0, 0 otherwise.",
        "0 !",
        |interpreter| {
            let a = interpreter.pop_bool()?;
            if !a {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_returns_true() {
        let mut interpreter = Interpreter::new(());
        let code = "0 !";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn not_returns_false() {
        let mut interpreter = Interpreter::new(());
        let code = "1 !";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn or_returns_true() {
        let mut interpreter = Interpreter::new(());
        let code = "0 1 ||";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn or_returns_false() {
        let mut interpreter = Interpreter::new(());
        let code = "0 0 ||";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn and_returns_true() {
        let mut interpreter = Interpreter::new(());
        let code = "1 1 &&";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn and_returns_false() {
        let mut interpreter = Interpreter::new(());
        let code = "0 1 &&";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }
}
