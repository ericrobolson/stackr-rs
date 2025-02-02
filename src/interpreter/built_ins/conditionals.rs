use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "else",
        "0|1 if .. else .. end -- ..",
        "Signals the end of an if statement and the start of an else statement.",
        "1 0 == if 1 else 0 end",
        |_| {
            // noop
            Ok(())
        },
    );

    interpreter.register_builtin(
        "end",
        "0|1 if .. end -- ..",
        "Word to end a conditional statement.",
        "1 1 == if 1 end",
        |_| {
            // noop
            Ok(())
        },
    );

    interpreter.register_builtin(
        "if",
        "0|1 if .. end -- ..",
        "If the top of the stack is true, execute the code until end is reached.",
        "1 1 == if 1 end",
        |interpreter| {
            let value = interpreter.pop_bool()?;

            // Get the end and else addresses
            let end_address = interpreter.address_cache.end_statement;
            let else_address = interpreter.address_cache.else_statement;

            // Setup the pc to start searching from
            let pc = interpreter.program_counter + 1;

            // Setup the end and else pc to None
            let mut end_pc = None;
            let mut else_pc = None;

            for idx in pc..interpreter.program.len() {
                let instruction = interpreter.program[idx].clone();
                if Some(end_address) == instruction.get_address() {
                    end_pc = Some(idx);
                    break;
                } else if Some(else_address) == instruction.get_address() && else_pc.is_none() {
                    else_pc = Some(idx);
                }
            }

            let end_pc = match end_pc {
                Some(pc) => pc,
                None => {
                    return Err((
                        "'end' statement not found".into(),
                        interpreter.get_location(),
                    ));
                }
            };

            // Only need to execute if true, otherwise skip
            if value {
                // If there's an else statement, that's our end.
                let end_pc = if let Some(else_pc) = else_pc {
                    else_pc
                } else {
                    end_pc
                };

                // Execute the code
                for i in pc..end_pc {
                    interpreter.execute_instruction(interpreter.program[i].clone())?;
                    interpreter.program_counter += 1;
                }
            } else if let Some(else_pc) = else_pc {
                // We're using an else statement, so skip the truthy part
                // of the if statement
                for i in (else_pc + 1)..end_pc {
                    interpreter.execute_instruction(interpreter.program[i].clone())?;
                    interpreter.program_counter += 1;
                }
            }

            // Skip to the end of the if statement
            interpreter.program_counter = end_pc;

            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_without_end_returns_error() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        1 1 == 
        if 
            "it's one!" 
        
        "#;
        let result: Result<(), (String, Location)> = interpreter.evaluate(code, None);
        assert_eq!(
            result,
            Err((
                "'end' statement not found".into(),
                interpreter.get_location()
            ))
        );
    }

    #[test]
    fn if_executes_code_if_true() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        1 1 == 
        if 
            "it's one!" 
        end
        "#;
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!("it's one!", interpreter.pop_string().unwrap());
    }

    #[test]
    fn if_skips_code_if_false() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        0 1 == 
        if 
            "it's one!" 
        end
        "#;
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!(true, interpreter.stack.is_empty());
    }

    #[test]
    fn if_skips_else_if_true() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        1 1 == 
        if 
            "it's one!" 
        else
            "it's not one!"
        end

        "after the end works"
        "#;
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!("after the end works", interpreter.pop_string().unwrap());
        assert_eq!("it's one!", interpreter.pop_string().unwrap());
    }

    #[test]
    fn if_skips_to_else_if_false() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        0 1 == 
        if 
            "it's one!" 
        else
            "it's not one!"
        end
        "#;
        let result = interpreter.evaluate(code, None);
        assert_eq!(result, Ok(()));
        assert_eq!("it's not one!", interpreter.pop_string().unwrap());
    }
}
