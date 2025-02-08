use super::*;

impl<State> Interpreter<State> {
    /// Resets the program counter and clears the program and program debug locations but preserves RAM and stack.
    #[allow(dead_code)]
    pub fn reset_program(&mut self) {
        self.program_counter = 0;
        self.program.clear();
        self.program_debug_locations.clear();
    }

    /// Load a program into the interpreter.
    pub(crate) fn load_program(&mut self, code: &str, path: Option<PathBuf>) -> Result<(), Err> {
        let mut location = Location::new(path);

        let mut maybe_make_word = |buffer: &mut String, word_location: &Location| {
            if !buffer.is_empty() {
                let word = buffer.trim();
                let location = word_location.clone();
                self.program_debug_locations.push(location);

                // Try to parse number
                let instruction = if let Ok(number) = word.parse::<Number>() {
                    Instruction::PushNumber(number)
                } else if word.starts_with('"') {
                    // Remove the first and last character
                    let mut word = word.to_string();
                    word.remove(0);
                    word.pop();
                    Instruction::PushString(word)
                } else {
                    Instruction::Address(self.get_address(word))
                };
                self.program.push(instruction);
                *buffer = String::new();
            }
        };

        let mut buffer = String::new();
        let mut word_location = location.clone();
        let mut making_string = false;
        let mut made_string = false;

        for c in code.chars() {
            if c == '"' {
                if !making_string {
                    making_string = true;
                } else {
                    let last_char = buffer.chars().last();
                    if last_char != Some('\\') {
                        making_string = false;
                        made_string = true;
                        buffer.push(c);
                    }
                }
            }
            if !making_string && c.is_whitespace() || made_string {
                made_string = false;
                maybe_make_word(&mut buffer, &word_location);
            } else {
                // set word location to current location if buffer is empty
                // as this is the first character of the word
                if buffer.is_empty() {
                    word_location = location.clone();
                }
                buffer.push(c);
            }

            // Update location
            if c == '\n' {
                location.new_line();
            } else {
                location.next();
            }
        }

        if making_string {
            return Err(("Unclosed string".into(), word_location));
        }

        // Handle case if buffer is not empty
        maybe_make_word(&mut buffer, &word_location);

        Ok(())
    }

    /// Execute the program.
    pub(crate) fn execute(&mut self) -> Result<(), Err> {
        while self.program_counter < self.program.len() {
            let instruction = self.program[self.program_counter].clone();
            self.execute_instruction(instruction)?;
            self.program_counter += 1;
        }

        Ok(())
    }

    /// Execute an instruction.
    pub(crate) fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), Err> {
        match instruction {
            Instruction::PushString(string) => self.push_string(string),
            Instruction::PushNumber(number) => self.push_number(number),
            Instruction::Address(address) => {
                // If we are in read mode, we only want to push the address if it is not the end of the read mode
                if self.read_mode != ReadMode::Off && address != self.address_cache.read_mode_end {
                    if self.read_mode == ReadMode::SingleWord {
                        self.read_mode = ReadMode::Off;

                        // Only insert a new address if it is not already in the ram
                        if !self.ram.contains_key(&address) {
                            let variable_location = self.next_address;
                            self.next_address = self.next_address.next();
                            self.ram
                                .insert(address, RamValue::Address(variable_location));
                        }
                    } else {
                        self.push_address(address);
                    }
                } else {
                    // Get address contents from memory and execute
                    let contents = self.ram.get(&address).ok_or((
                        format!("Address not found: {}", self.get_name(address)),
                        self.location(),
                    ))?;
                    let ops_to_execute = match contents {
                        RamValue::Compiled(ops) => Some(ops.clone()),
                        RamValue::BuiltIn(method) => {
                            method(self)?;
                            None
                        }
                        RamValue::Value(value) => {
                            self.stack.push(StackValue::Value(value.clone()));
                            None
                        }
                        RamValue::Address(address) => {
                            self.stack.push(address.into());
                            None
                        }
                    };
                    if let Some(ops) = ops_to_execute {
                        for op in ops {
                            self.execute_instruction(op)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
