use super::*;

/// An instruction that can be executed by the interpreter.
#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    /// Push a number onto the stack.
    PushNumber(Number),
    /// Push a string onto the stack.
    PushString(String),
    /// Push an address onto the stack or evaluate it.
    Address(Address),
}

impl Instruction {
    /// Display the type of the instruction.
    pub fn display_type<State>(&self, interpreter: &Interpreter<State>) -> String {
        match self {
            Instruction::PushNumber(n) => format!("N{}", n),
            Instruction::PushString(s) => format!("\"{}\"", s),
            Instruction::Address(a) => format!("{}", interpreter.get_name(*a)),
        }
    }

    pub fn get_address(&self) -> Option<Address> {
        match self {
            Instruction::Address(a) => Some(*a),
            _ => None,
        }
    }

    /// Expect an address from the instruction.
    pub fn expect_address<State>(&self, interpreter: &Interpreter<State>) -> Result<Address, Err> {
        match self {
            Instruction::Address(address) => Ok(*address),
            _ => Err((
                format!(
                    "Expected an address, got {}",
                    self.display_type(interpreter)
                ),
                interpreter.location(),
            )),
        }
    }

    /// Expect a number from the instruction.
    #[allow(unused)]
    pub fn expect_number<State>(&self, interpreter: &Interpreter<State>) -> Result<Number, Err> {
        match self {
            Instruction::PushNumber(number) => Ok(*number),
            _ => Err((
                format!("Expected a number, got {}", self.display_type(interpreter)),
                interpreter.location(),
            )),
        }
    }

    /// Expect a string from the instruction.
    pub fn expect_string<State>(&self, interpreter: &Interpreter<State>) -> Result<String, Err> {
        match self {
            Instruction::PushString(string) => Ok(string.clone()),
            _ => Err((
                format!("Expected a string, got {}", self.display_type(interpreter)),
                interpreter.location(),
            )),
        }
    }
}
