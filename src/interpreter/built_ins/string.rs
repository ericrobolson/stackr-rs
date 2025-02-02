use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "concat",
        "string string -- string",
        "Concatenates two strings.",
        "\"hello\" \" world\" concat",
        |interpreter| {
            let b = interpreter.pop_string()?;
            let a = interpreter.pop_string()?;
            interpreter.push_string(format!("{}{}", a, b));
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_over() {
        let mut interpreter = Interpreter::new(());
        let code = "\"hello\" \" world\" concat";
        interpreter.evaluate(code, None).unwrap();
        assert_eq!(interpreter.pop_string().unwrap(), "hello world");
    }
}
