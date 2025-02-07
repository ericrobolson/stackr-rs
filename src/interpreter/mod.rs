/// Addresses used to identify words in the interpreter.
mod address;
/// Addresses used to identify words in the interpreter.
mod address_cache;
/// Built-in words registered at runtime.
mod built_ins;
/// Contains logic for loading and executing a program.
mod evaluate;
/// Instructions that can be executed by the interpreter.
mod instruction;
/// Locations of the program instructions. Used for debugging.
mod location;
/// Values stored in the RAM.
mod ram_value;
/// Values stored on the stack.
mod stack_value;
/// Stringifies the program.
mod stringify;

pub(crate) use address_cache::*;
pub(crate) use instruction::*;
pub(crate) use ram_value::*;
pub use stack_value::*;

pub use address::*;
pub use built_ins::*;
pub use location::*;

use std::{collections::HashMap, io::Write, path::PathBuf};

/// A type alias for an error.
pub type Err = (String, Location);
/// A type alias for a number.
pub type Number = f32;

/// A value that is stored in RAM or on the stack.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(Number),
    String(String),
}

/// The mode of the interpreter.
/// Read mode determines if words should be read as addresses or evaluated.
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum ReadMode {
    On,
    Off,
    SingleWord,
}

/// The core interpreter object.
/// Takes in a custom defined state that can be modified by built-ins.
pub struct Interpreter<State> {
    /// Custom state for each application.
    /// Allows users to register built-ins that modify this state.
    #[allow(unused)]
    pub state: State,
    /// Address cache. Used to remove need to lookup addresses every time.
    address_cache: AddressCache,
    /// Whether the program should break out of the current loop.
    break_loop: bool,
    /// Program counter.
    program_counter: usize,
    /// Program to execute.
    program: Vec<Instruction>,
    /// Program counter stack. Used for loops/control flow.
    program_counter_stack: Vec<usize>,
    /// Whether the interpreter is in compile mode.
    compiling: bool,
    /// Locations of the program instructions. Used for debugging.
    program_debug_locations: Vec<Location>,
    /// Whether the interpreter should quit.
    exit: bool,
    /// Read mode. Determines if words should be read as addresses or evaluated.
    read_mode: ReadMode,
    /// Whether the interpreter should run a REPL.
    repl_mode: bool,
    /// Next address to use.
    next_address: Address,
    /// RAM. Stores values and words.
    ram: HashMap<Address, RamValue<State>>,
    /// Stack.
    stack: Vec<StackValue>,
    /// Documentation table. Maps addresses to documentation.
    documentation_table: HashMap<Address, String>,
    /// Name table. Maps names to addresses.
    name_table: HashMap<String, Address>,
}

impl<State> Interpreter<State> {
    /// Create a new interpreter.
    /// Pass in the initial custom state for the interpreter.
    pub fn new(state: State) -> Self {
        let mut interpreter = Self {
            state,
            compiling: false,
            break_loop: false,
            address_cache: AddressCache::uninitalized(),
            program_counter: 0,
            program: vec![],
            program_counter_stack: vec![],
            program_debug_locations: vec![],
            exit: false,
            read_mode: ReadMode::Off,
            repl_mode: false,
            ram: HashMap::new(),
            stack: vec![],
            next_address: Address::default(),
            documentation_table: HashMap::new(),
            name_table: HashMap::new(),
        };

        interpreter.register_builtins();

        AddressCache::initialize(&mut interpreter);

        interpreter
    }

    /// Evaluate a program.
    pub fn evaluate(&mut self, code: &str, path: Option<PathBuf>) -> Result<(), Err> {
        self.load_program(code, path)?;
        self.execute()?;
        Ok(())
    }

    /// Get the name of an address if present.
    pub fn get_name(&self, address: Address) -> String {
        // Check if the address is in the name table
        for (name, addr) in &self.name_table {
            if *addr == address {
                return name.clone();
            }
        }

        // Didn't find it, so if we have something in ram that contains the address, use that
        for (k, value) in &self.ram {
            if let Some(addr) = value.address() {
                if addr == address {
                    return format!("@{}", self.get_name(*k));
                }
            }
        }

        // Didn't find it, so return an unknown name
        format!("@UNKNOWN-{:?}", address)
    }

    /// Pop a value from the stack.
    pub fn pop(&mut self) -> Result<StackValue, Err> {
        self.stack
            .pop()
            .ok_or(("Stack is empty".to_string(), self.location()))
    }

    /// Pop a boolean from the stack.
    pub fn pop_bool(&mut self) -> Result<bool, Err> {
        match self.pop()? {
            StackValue::Value(Value::Number(number)) => Ok(number != 0.0),
            _ => Err(("Expected a boolean/number".to_string(), self.location())),
        }
    }

    /// Pop a number from the stack.
    pub fn pop_number(&mut self) -> Result<f32, Err> {
        match self.pop()? {
            StackValue::Value(Value::Number(number)) => Ok(number),
            _ => Err(("Expected a number".to_string(), self.location())),
        }
    }

    /// Push a number onto the stack.
    pub fn push_number(&mut self, number: f32) {
        self.stack.push(StackValue::Value(Value::Number(number)));
    }

    /// Push an address onto the stack.
    pub fn push_address(&mut self, address: Address) {
        self.stack.push(StackValue::Address(address));
    }

    /// Pop an address from the stack.
    pub fn pop_address(&mut self) -> Result<Address, Err> {
        match self.pop()? {
            StackValue::Address(address) => Ok(address),
            _ => Err(("Expected an address".to_string(), self.location())),
        }
    }

    /// Push a string onto the stack.
    pub fn push_string(&mut self, string: String) {
        self.stack.push(StackValue::Value(Value::String(string)));
    }

    /// Pop a string from the stack.
    pub fn pop_string(&mut self) -> Result<String, Err> {
        match self.pop()? {
            StackValue::Value(Value::String(string)) => Ok(string),
            _ => Err(("Expected a string".to_string(), self.location())),
        }
    }

    /// Start a REPL.
    pub fn start_repl(&mut self) -> Result<(), Err> {
        self.repl_mode = true;
        println!("Use 'repl-exit' to exit REPL mode.");

        while !self.exit && self.repl_mode {
            // Read input from stdin
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            let input = format!("{} print-stack", input.trim());

            // Evaluate input
            match self.evaluate(&input, None) {
                Ok(_) => (),
                Err((e, location)) => println!("{}: {}", location, e),
            }
        }
        Ok(())
    }

    /// Register a built-in function to be used in the interpreter.
    pub fn register_builtin(
        &mut self,
        name: &str,
        stack_modification: &str,
        documentation: &str,
        example: &str,
        func: BuiltIn<State>,
    ) {
        let address = self.get_address(name);
        self.register_documentation(address, stack_modification, documentation, example);
        self.ram.insert(address, RamValue::BuiltIn(func));
    }

    /// Register documentation for a word.
    pub(crate) fn register_documentation(
        &mut self,
        address: Address,
        stack_modification: &str,
        documentation: &str,
        example: &str,
    ) {
        let stack_modification = if stack_modification.is_empty() {
            "".to_string()
        } else {
            format!("\t( {} )", stack_modification.replace("...", "..").trim())
        };

        let example = if example.is_empty() {
            "".to_string()
        } else {
            format!("\n\tExample '{}'", example)
        };
        let documentation = documentation.trim();
        let documentation = format!("{stack_modification}\n\t{}{}", documentation, example);

        self.documentation_table.insert(address, documentation);
    }

    /// Print the documentation for all words.
    pub fn print_documentation(&self) {
        println!("Documentation:");
        let mut names = self.name_table.keys().cloned().collect::<Vec<_>>();
        names.sort();
        for name in names {
            let address = self.name_table[&name];
            if let Some(documentation) = self.documentation_table.get(&address) {
                println!("{}  {}\n", name, documentation);
            }
        }
        println!();
    }

    /// Get the address of a name.
    pub(crate) fn get_address(&mut self, name: &str) -> Address {
        if let Some(address) = self.name_table.get(name) {
            address.clone()
        } else {
            let address = self.next_address;
            self.next_address = self.next_address.next();
            self.name_table.insert(name.to_string(), address);
            address
        }
    }

    /// Get the next instruction from the program.
    pub(crate) fn chomp_instruction(&mut self) -> Result<Instruction, Err> {
        if self.program_counter >= self.program.len() {
            return Err(("No more instructions".to_string(), self.location()));
        }

        let instruction = self.program[self.program_counter].clone();
        self.program_counter += 1;
        Ok(instruction)
    }

    /// Returns the location of the current instruction.
    pub fn location(&self) -> Location {
        if self.program_debug_locations.is_empty()
            || self.program_counter >= self.program_debug_locations.len()
        {
            return Location::default();
        }
        self.program_debug_locations[self.program_counter].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_address() {
        let mut interpreter = Interpreter::new(());
        let expected = interpreter.next_address;
        let actual = interpreter.get_address("test");
        assert_eq!(expected, actual);
    }

    #[test]
    fn pop_returns_err_if_stack_is_empty() {
        let mut interpreter = Interpreter::new(());
        let err = interpreter.pop().unwrap_err();
        assert_eq!(err, ("Stack is empty".to_string(), Location::default()));
    }

    #[test]
    fn pop_number_returns_err_if_not_number() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_address(Address::default());
        let err = interpreter.pop_number().unwrap_err();
        assert_eq!(err, ("Expected a number".to_string(), Location::default()));
    }

    #[test]
    fn pop_bool_returns_err_if_not_bool() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_string("hello".to_string());
        let err = interpreter.pop_bool().unwrap_err();
        assert_eq!(
            err,
            ("Expected a boolean/number".to_string(), Location::default())
        );
    }

    #[test]
    fn pop_bool_returns_true() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_number(0.1);
        let value = interpreter.pop_bool().unwrap();
        assert_eq!(value, true);
    }

    #[test]
    fn pop_bool_returns_false() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_number(0.0);
        let value = interpreter.pop_bool().unwrap();
        assert_eq!(value, false);
    }

    #[test]
    fn pop_number_returns_value() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1", None).unwrap();
        let value = interpreter.pop_number().unwrap();
        assert_eq!(value, 1.0);
    }

    #[test]
    fn pop_string_returns_err_if_not_string() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1", None).unwrap();
        let err = interpreter.pop_string().unwrap_err();
        assert_eq!(err, ("Expected a string".to_string(), Location::default()));
    }

    #[test]
    fn pop_string_returns_value() {
        let mut interpreter = Interpreter::new(());
        interpreter.push_string("hello".to_string());
        let value = interpreter.pop_string().unwrap();
        assert_eq!(value, "hello");
    }

    #[test]
    fn pop_address_returns_err_if_not_address() {
        let mut interpreter = Interpreter::new(());
        interpreter.evaluate("1", None).unwrap();

        let err = interpreter.pop_address().unwrap_err();
        assert_eq!(
            err,
            ("Expected an address".to_string(), Location::default())
        );
    }

    #[test]
    fn pop_address_returns_value() {
        let mut interpreter = Interpreter::new(());
        let address = interpreter.next_address;
        interpreter.push_address(address.clone());
        let value = interpreter.pop_address().unwrap();
        assert_eq!(value, address);
    }

    #[test]
    fn parse_string_returns_err_if_not_closed() {
        let code = r#""hello"#;
        let mut interpreter = Interpreter::new(());
        let result = interpreter.evaluate(code, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, "Unclosed string");
    }

    #[test]
    fn parse_string_puts_string_on_stack() {
        let code = r#""hello""#;

        let mut interpreter = Interpreter::new(());
        interpreter.evaluate(code, None).unwrap();
        let value = interpreter.pop_string().unwrap();
        assert_eq!(value, "hello");
    }

    // #[test]
    // fn parse_string_puts_escaped_string_on_stack() {
    //     let mut interpreter = Interpreter::new(());
    //     let code = "\"hello\n \\\" \"";
    //     interpreter.evaluate(code, None).unwrap();
    //     let value = interpreter.pop_string().unwrap();
    //     assert_eq!(value.as_str(), "hello\n \" ");
    // }
}
