use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    let example = ": squared \"squares a number\" \"n -- n\" \"2 squared\" * * ;";
    interpreter.register_builtin(
            ":",
            "$name \"documentation\" \"stack modification\" \"example\" .. --",
            "Begins compile mode. The following words are compiled into a function with the given $name. Use ';' to end.",
            example,
            |interpreter| {
                interpreter.compiling = true;
                // Step through program and find the call of the end compile address
                // Then iterate to get to that point and compile the function.
                let mut end_address_idx = None;
                for i in interpreter.program_counter..interpreter.program.len() {
                    match interpreter.program[i] {
                        Instruction::Address(address) => {
                            if address == interpreter.address_cache.compile_end {
                                end_address_idx = Some(i);
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                let end_address_idx = match end_address_idx {
                    Some(idx) => idx,
                    None => return Err((
                        "No ; found, unable to compile".into(),
                        interpreter.location(),
                    )),
                };

                // Skip the ':'
                interpreter.chomp_instruction()?;
                let name = interpreter.chomp_instruction()?.expect_address(interpreter)?;
                let documentation = interpreter.chomp_instruction()?.expect_string(interpreter)?;
                let stack_modification = interpreter.chomp_instruction()?.expect_string(interpreter)?;
                let example = interpreter.chomp_instruction()?.expect_string(interpreter)?;
                
                interpreter.register_documentation(name, &stack_modification, &documentation, &example);
                
                // Compile the function
                let mut idx = interpreter.program_counter;
                let mut ops = Vec::new();
                while idx < end_address_idx {
                    ops.push(interpreter.program[idx].clone());
                    idx += 1;
                }

                interpreter.ram.insert(name, RamValue::Compiled(ops));
                
                // Reset compiler state and update program counter.
                interpreter.program_counter = end_address_idx;
                interpreter.compiling = false;
                Ok(())
            },
        );

    interpreter.register_builtin(
        ";",
        "",
        "Ends compile mode. The function is added to the program.",
        example,
        |interpreter| {
            interpreter.compiling = false;
            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_no_end_address_returns_error() {
        let code = r#"
        : squared "squares a number" "n -- n" "2 squared" * * 
        "#;

        let mut interpreter = Interpreter::new(());
        let result = interpreter.evaluate(code, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, "No ; found, unable to compile");
    }

    #[test]
    fn compile_and_execute() {
        let code = r#"
        : mul2 "multiplies a number by 2" "n -- n" "2 mul2" 2 * ;
        2 mul2
        "#;

        let mut interpreter = Interpreter::new(());
        interpreter.evaluate(code, None).unwrap();

        assert_eq!(4.0, interpreter.pop_number().unwrap());
    }
}
