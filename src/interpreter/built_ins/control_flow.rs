use super::*;

pub fn register_builtins<State>(interpreter: &mut Interpreter<State>) {
    interpreter.register_builtin(
        "loop",
        "--",
        "Signals the end of a loop and resumes control flow at the next relevant 'loop' word.",
        "0 begin 1 + if 10 > break end loop",
        |interpreter| {
            if interpreter.break_loop {
                interpreter.break_loop = false;
                return Ok(());
            }

            if interpreter.program_counter_stack.is_empty() {
                return Err((
                    "'begin' statement not found".into(),
                    interpreter.location(),
                ));
            }

            let begin_pc = interpreter.program_counter_stack.pop().unwrap();
            interpreter.program_counter = begin_pc;
            interpreter.program_counter_stack.push(begin_pc);
            Ok(())
        },
    );

    interpreter.register_builtin("break", "", "", "", |interpreter| {
        // Find next loop statement and jump to it
        // Setup the pc to start searching from
        let pc = interpreter.program_counter + 1;
        let loop_address = interpreter.address_cache.loop_statement;
        let mut end_pc = None;
        for idx in pc..interpreter.program.len() {
            let instruction = interpreter.program[idx].clone();
            if Some(loop_address) == instruction.get_address() {
                end_pc = Some(idx);
                break;
            }
        }

        match end_pc {
            Some(pc) => {
                interpreter.program_counter_stack.pop();
                interpreter.program_counter = pc;
                interpreter.break_loop = true;
            }
            None => {
                return Err((
                    "'loop' statement not found".into(),
                    interpreter.location(),
                ));
            }
        }

        Ok(())
    });

    interpreter.register_builtin(
        "begin",
        "--",
        "Starts a loop. 'break' must be called to end it.",
        "0 begin 1 + if 10 > break end loop",
        |interpreter| {
            let begin_pc = interpreter.program_counter;
            interpreter.program_counter_stack.push(begin_pc);

            Ok(())
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_returns_err_if_no_begin() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        1 1 + loop
        "#;

        let result = interpreter.evaluate(code, None);
        assert_eq!(
            result,
            Err((
                "'begin' statement not found".into(),
                interpreter.location()
            ))
        );
    }

    #[test]
    fn loop_pops_program_counter_and_resumes_from_begin() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        begin 1 1 + loop
        "#;

        // Setup the interpreter so it thinks it's at the loop statement
        // with begin on the stack
        interpreter.program_counter = 3;
        interpreter.program_counter_stack.push(0);

        interpreter.load_program(code, None).unwrap();
        let instruction = Instruction::Address(interpreter.get_address("loop"));
        interpreter.execute_instruction(instruction).unwrap();

        assert_eq!(interpreter.program_counter, 0);
        assert_eq!(interpreter.program_counter_stack, vec![0]);
    }

    #[test]
    fn break_without_loop_returns_err() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        begin 
            "hello"
            break
        "#;

        let result = interpreter.evaluate(code, None);
        assert_eq!(
            result,
            Err((
                "'loop' statement not found".into(),
                interpreter.location()
            ))
        );
    }

    #[test]
    fn loop_executes_until_break() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        begin 
            "hello"
            break
        loop

        "world"
        "#;

        interpreter.evaluate(code, None).unwrap();
        assert_eq!("world", interpreter.pop_string().unwrap());
        assert_eq!("hello", interpreter.pop_string().unwrap());
    }

    #[test]
    fn loop_until_break_in_if() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        0
        begin
            1 + dup

            2 == if 
                "hello" 
                break
            end
        loop

        "world"
        "#;

        interpreter.evaluate(code, None).unwrap();
        assert_eq!("world", interpreter.pop_string().unwrap());
        assert_eq!("hello", interpreter.pop_string().unwrap());
    }

    #[test]
    #[ignore]
    fn nested_loop_executes() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
begin 
    "hello"

    0
    begin
        1 + dup
        print-stack

        
        if 10 == 
            print-stack
            break 
        end
       
    loop

    "after loop"
    print-stack

    break
loop
"!"    
        "#;

        interpreter.evaluate(code, None).unwrap();
        assert_eq!("!", interpreter.pop_string().unwrap());
        assert_eq!("world", interpreter.pop_string().unwrap());
        assert_eq!("hello", interpreter.pop_string().unwrap());
    }
}
