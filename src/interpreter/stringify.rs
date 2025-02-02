use super::*;

const MAX_TOKENS_ON_LINE: usize = 8;
struct ProgramTokens {
    tokens: Vec<String>,
    indent_count: usize,
    buffer: String,
    current_tokens_on_line: usize,
}
impl ProgramTokens {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            tokens: vec![],
            indent_count: 0,
            current_tokens_on_line: 0,
        }
    }

    pub fn push(&mut self, token: String) {
        self.tokens.push(token);
    }

    fn last_char(&self) -> Option<char> {
        if self.buffer.is_empty() {
            return None;
        }
        Some(self.buffer.chars().last().unwrap())
    }

    pub fn indent(&mut self) {
        self.indent_count += 1;
    }

    pub fn dedent(&mut self) {
        if self.indent_count > 0 {
            self.indent_count -= 1;
        }
    }

    pub fn add_space(&mut self) {
        self.buffer.push(' ');
    }

    pub fn chomp(&mut self) {
        if self.tokens.is_empty() {
            return;
        }
        self.buffer.push_str(&self.tokens.remove(0));
        self.current_tokens_on_line += 1;
        if self.current_tokens_on_line >= MAX_TOKENS_ON_LINE {
            self.add_newline();
        }
    }

    pub fn peek(&self) -> Option<String> {
        if self.tokens.is_empty() {
            return None;
        }
        Some(self.tokens[0].clone())
    }

    pub fn add_newline(&mut self) {
        self.buffer.push('\n');
        for _ in 0..self.indent_count {
            self.buffer.push('\t');
        }
        self.current_tokens_on_line = 0;
    }
}

impl<State> Interpreter<State> {
    /// Format a code file.
    pub fn format_code(code: &str, path: Option<PathBuf>) -> Result<String, Err> {
        let mut interpreter = Interpreter::new(());
        interpreter.load_program(code, path)?;
        Ok(interpreter.stringify_program())
    }

    /// Returns the program as a formatted string.
    pub fn stringify_program(&self) -> String {
        // Tokenize the program
        let mut tokens = ProgramTokens::new();
        let mut idx = 0;
        while idx < self.program.len() {
            let instruction = self.program[idx].clone();

            match instruction {
                Instruction::PushNumber(n) => tokens.push(format!("{}", n)),
                Instruction::PushString(s) => tokens.push(format!("\"{}\"", s)),
                Instruction::Address(address) => {
                    tokens.push(self.get_name(address));
                }
            }

            idx += 1;
        }

        while let Some(token) = tokens.peek() {
            match token.as_str() {
                // Function definition
                ":" => {
                    tokens.chomp();
                    tokens.add_space();
                    tokens.chomp();
                    tokens.indent();
                    tokens.add_newline();

                    // Documentation stuff
                    for _ in 0..3 {
                        tokens.chomp();
                        tokens.add_newline();
                    }
                    tokens.add_newline();
                }
                // Function definition end
                ";" => {
                    tokens.dedent();
                    tokens.add_newline();
                    tokens.chomp();
                    tokens.add_newline();
                    tokens.add_newline();
                }
                // Read mode start
                "[" => {
                    tokens.add_newline();
                    tokens.chomp();
                    tokens.indent();
                    tokens.add_newline();
                }
                // Read mode end
                "]" => {
                    tokens.dedent();
                    tokens.add_newline();
                    tokens.chomp();

                    let mut add_newline = true;
                    if let Some(token) = tokens.peek() {
                        if token == ";" {
                            add_newline = false;
                        }
                    }

                    if add_newline {
                        tokens.add_newline();
                    }
                }
                "begin" | "if" => {
                    tokens.add_newline();
                    tokens.chomp();
                    tokens.indent();
                    tokens.add_newline();
                }
                "else" => {
                    tokens.dedent();
                    tokens.add_newline();
                    tokens.chomp();
                    tokens.indent();
                    tokens.add_newline();
                }
                "loop" | "end" => {
                    tokens.dedent();
                    tokens.add_newline();
                    tokens.chomp();

                    if Some("loop".to_string()) == tokens.peek() {
                    } else {
                        tokens.add_newline();
                        tokens.add_newline();
                    }
                }
                "break" => {
                    tokens.add_newline();
                    tokens.chomp();
                }
                _ => {
                    if Some("begin".to_string()) == tokens.peek()
                        || Some("if".to_string()) == tokens.peek()
                    {
                        tokens.add_newline();
                    } else if Some('\n') != tokens.last_char() && Some('\t') != tokens.last_char() {
                        tokens.add_space();
                    }
                    tokens.chomp();
                }
            }
        }

        let mut buffer = tokens.buffer.trim().to_string();
        buffer.push('\n');
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to assert that two strings are equal.
    /// Provides debugging information if the assertion fails.
    fn assert_equal(expected: &str, actual: &str) {
        println!("EXPECTED START");
        println!("{}", expected);
        println!("EXPECTED END\n");
        println!("ACTUAL START");
        println!("{}", actual);
        println!("ACTUAL END\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn stringify_program_returns_program_as_string() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1 2 +", None).unwrap();
        let program = interpreter.stringify_program();

        assert_equal("1 2 +\n", &program);
    }

    #[test]
    fn stringify_complex_program_returns_program_as_string() {
        let mut interpreter = Interpreter::new(());
        let code = r#"
        : square 
            "stack modification" 
            "documentation"  "example" dup * ; : complextro "stack modification" "documentation" "example" square 3 [ dup * square ] ; 2 square
        "#;
        interpreter.evaluate(code, None).unwrap();
        let program = interpreter.stringify_program();
        let expected = ": square\n\t\"stack modification\"\n\t\"documentation\"\n\t\"example\"\n\t\n\tdup *\n;\n\n: complextro\n\t\"stack modification\"\n\t\"documentation\"\n\t\"example\"\n\t\n\tsquare 3\n\t[\n\t\tdup * square\n\t]\n;\n\n2 square\n";

        assert_equal(expected, &program);
    }

    #[test]
    fn stringify_single_loop_and_if_returns_program_as_string() {
        let mut interpreter = Interpreter::new(());

        let code = r#"
        : debug "" "" "" print-stack drop ;
        0
        begin
            1 + dup

            2 == if 
                "hello" 
                break
            end

            dup
            2 == if 
                "hello"
            else
                "world"
                drop
            end
        loop

        "world"
        
        "#;
        interpreter.evaluate(code, None).unwrap();
        let program = interpreter.stringify_program();
        let expected = ": debug\n\t\"\"\n\t\"\"\n\t\"\"\n\t\n\tprint-stack drop\n;\n\n0\nbegin\n\t1 + dup 2 ==\n\tif\n\t\t\"hello\"\n\t\tbreak\n\tend\n\t\n\tdup 2 ==\n\tif\n\t\t\"hello\"\n\telse\n\t\t\"world\" drop\n\tend\nloop\n\n\"world\"\n";

        assert_equal(expected, &program);
    }
}
