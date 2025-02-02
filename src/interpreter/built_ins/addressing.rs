use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "var",
        "var $name --",
        "Declares a variable. Calling the $name will return the address of the variable.",
        "var life \t 42 life set",
        |interpreter| {
            interpreter.read_mode = ReadMode::SingleWord;
            Ok(())
        },
    );

    interpreter.register_builtin(
        "set",
        "<value> $name set --",
        "Sets the value of a variable to the top of the stack.",
        "42 life set",
        |interpreter| {
            let name = interpreter.pop_address()?;
            let value = interpreter.pop()?;

            interpreter.ram.insert(name, value.into());
            Ok(())
        },
    );

    interpreter.register_builtin(
        "get",
        "@name get -- <value>",
        "Gets the value of a variable and puts it on the stack.",
        "life get",
        |interpreter| {
            let name = interpreter.pop_address()?;
            match interpreter.ram.get(&name) {
                Some(RamValue::Value(value)) => {
                    interpreter.stack.push(value.clone().into());
                }
                Some(RamValue::Address(address)) => {
                    interpreter.stack.push(address.into());
                }
                Some(RamValue::Compiled(_)) => {
                    todo!("Implement get for compiled");
                }
                Some(RamValue::BuiltIn(_)) => {
                    // TODO: might be able to handle gracefully,
                    // idk how right now though.
                    let name = interpreter.get_name(name);
                    panic!("Can't get value of built in function '{}'", name);
                }
                None => {
                    return Err(("Unknown address".to_string(), interpreter.location()));
                }
            }
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_puts_address_on_stack() {
        let mut interpreter = Interpreter::new(());
        interpreter.register_builtins();
        interpreter.evaluate("var hi hi", None).unwrap();
        let address = interpreter.get_address("hi");
        let expected = interpreter.ram.get(&address).unwrap().expect_address();
        assert_eq!(interpreter.stack, [expected.into()]);
    }

    #[test]
    fn var_puts_single_address_on_stack() {
        let mut interpreter = Interpreter::new(());
        interpreter.register_builtins();
        interpreter.evaluate("var hi 3 3 + hi", None).unwrap();
        let address = interpreter.get_address("hi");
        let expected = interpreter.ram.get(&address).unwrap().expect_address();
        assert_eq!(interpreter.stack, [6.0.into(), expected.into()]);
    }

    #[test]
    fn set_sets_value_of_variable() {
        let mut interpreter = Interpreter::new(());

        let code = r#"
        var life
        42 life set
        "#;
        interpreter.evaluate(code, None).unwrap();
        let address = interpreter.get_address("life");
        let value_address = interpreter.ram.get(&address).unwrap().expect_address();
        let value = interpreter.ram.get(&value_address).unwrap().expect_value();
        assert_eq!(value, Value::Number(42.0));
    }

    #[test]
    fn get_returns_value_of_variable_number() {
        let mut interpreter = Interpreter::new(());

        let code = r#"
        var life
        42 life set
        life get
        "#;
        interpreter.evaluate(code, None).unwrap();
        let value = interpreter.pop_number().unwrap();
        assert_eq!(value, 42.0);
    }

    #[test]
    fn get_returns_value_of_variable_address() {
        let mut interpreter = Interpreter::new(());

        let code = r#"
        var life
        var life2
        life2 life set
        life get
        "#;
        interpreter.evaluate(code, None).unwrap();
        let expected = interpreter.get_address("life2");
        let expected = interpreter.ram.get(&expected).unwrap().expect_address();

        let actual = interpreter.pop_address().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_on_underflow_returns_err() {
        let mut interpreter = Interpreter::new(());
        let result = interpreter.evaluate(" get", None);
        assert!(result.is_err());
        let (err, _) = result.unwrap_err();
        assert_eq!(err, "Stack is empty");
    }

    #[test]
    fn get_on_nothing_returns_err() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::new(11110));
        let result = interpreter.evaluate(" get", None);
        assert!(result.is_err());
        let (err, _) = result.unwrap_err();
        assert_eq!(err, "Unknown address");
    }
}
