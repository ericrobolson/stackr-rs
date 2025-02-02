use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "==",
        "any any -- 0|1",
        "Checks if two values are equal. Puts 1 on the stack if they are equal, 0 otherwise.",
        "1 1 ==",
        |interpreter| {
            let a = interpreter.pop()?;
            let b = interpreter.pop()?;
            if a == b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        "!=",
        "any any -- 0|1",
        "Checks if two values are not equal. Puts 1 on the stack if they are not equal, 0 otherwise.",
        "1 2 !=",
        |interpreter| {
            let a = interpreter.pop()?;
            let b = interpreter.pop()?;
            if a != b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        ">",
        "n n -- 0|1",
        "Checks if the second number is greater than the first. Puts 1 on the stack if it is, 0 otherwise.",
        "1 2 >",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            if a > b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        ">=",
        "n n -- 0|1",
        "Checks if the second number is greater than or equal to the first. Puts 1 on the stack if it is, 0 otherwise.",
        "1 2 >=",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            if a >= b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        "<",
        "n n -- 0|1",
        "Checks if the second number is less than the first. Puts 1 on the stack if it is, 0 otherwise.",
        "1 2 <",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            if a < b {
                interpreter.push_number(1.0);
            } else {
                interpreter.push_number(0.0);
            }
            Ok(())
        },
    );

    interpreter.register_builtin(
        "<=",
        "n n -- 0|1",
        "Checks if the second number is less than or equal to the first. Puts 1 on the stack if it is, 0 otherwise.",
        "1 2 <=",
        |interpreter| {
            let a = interpreter.pop_number()?;
            let b = interpreter.pop_number()?;
            if a <= b {
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
    fn test_greater_than_equal_than_two_numbers_returns_1_if_greater_than() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 >=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_greater_than_equal_than_two_numbers_returns_1_if_equal() {
        let mut interpreter = Interpreter::new(());
        let code = "2 2 >=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_greater_than_equal_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "5 2 >=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_less_equal_than_two_numbers_returns_1_if_less_than() {
        let mut interpreter = Interpreter::new(());
        let code = "2 1 <=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_less_equal_than_two_numbers_returns_1_if_equal() {
        let mut interpreter = Interpreter::new(());
        let code = "2 2 <=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_less_than_equal_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 <=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_less_than_two_numbers_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "2 1 <";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_less_than_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 <";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_greater_than_two_numbers_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 >";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_greater_than_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "2 1 >";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_equal_two_numbers_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "1 1 ==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_equal_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "1 -1 ==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_equal_two_strings_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "\"hello\" \"hello\" ==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_equal_two_strings_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "\"bob says\" \"hello\" ==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_two_addresses_are_equal() {
        let mut interpreter = Interpreter::new(());
        let address = Address::new(1000);
        interpreter.push_address(address);
        interpreter.push_address(address);
        let code = "==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_two_addresses_are_not_equal() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(1020));
        interpreter.push_address(Address::new(1000));
        let code = "==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_equal_different_types_returns_0() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_number(1.0);
        interpreter.push_string("hello".to_string());
        let code = "==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());

        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(1000));
        interpreter.push_string("hello".to_string());
        let code = "==";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_numbers_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "1 2 !=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_numbers_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "1 1 !=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_strings_returns_1() {
        let mut interpreter = Interpreter::new(());
        let code = "\"bob says\" \"hello\" !=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_strings_returns_0() {
        let mut interpreter = Interpreter::new(());
        let code = "\"hello\" \"hello\" !=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_addresses_returns_1() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(1000));
        interpreter.push_address(Address::new(1030));
        let code = "!=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_two_addresses_returns_0() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(1000));
        interpreter.push_address(Address::new(1000));
        let code = "!=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(0.0, interpreter.pop_number().unwrap());
    }

    #[test]
    fn test_not_equal_different_types_returns_1() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_number(1.0);
        interpreter.push_string("hello".to_string());
        let code = "!=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());

        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(1000));
        interpreter.push_string("hello".to_string());
        let code = "!=";
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(1.0, interpreter.pop_number().unwrap());
    }
}
