use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    let example = "[ rot drop dup ]";
    interpreter.register_builtin(
            "[",
            ".. -- ..",
            "Begins read mode. The addresses of all following words are put on the stack. Use ']' to end.",
            example,
            |interpreter| {
                interpreter.read_mode = ReadMode::On;
                Ok(())
            },
        );

    interpreter.register_builtin(
        "]",
        "",
        "Ends read mode. All following words are evaluated.",
        example,
        |interpreter| {
            interpreter.read_mode = ReadMode::Off;
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_read_mode_true() {
        let mut interpreter = Interpreter::new(());
        interpreter.register_builtins();
        interpreter.evaluate("[", None).unwrap();
        assert_eq!(interpreter.read_mode, ReadMode::On);
    }

    #[test]
    fn sets_read_mode_false() {
        let mut interpreter = Interpreter::new(());
        interpreter.register_builtins();
        interpreter.evaluate("[ ]", None).unwrap();
        assert_eq!(interpreter.read_mode, ReadMode::Off);
    }

    #[test]
    fn read_mode_puts_addresses_on_stack() {
        let mut interpreter = Interpreter::new(());
        interpreter.register_builtins();
        interpreter.evaluate("[ hi there buddy ]", None).unwrap();

        let input = ["hi", "there", "buddy"];
        let expected = input
            .iter()
            .map(|s| StackValue::Address(interpreter.get_address(s)))
            .collect::<Vec<_>>();

        assert_eq!(expected, interpreter.stack);
    }
}
