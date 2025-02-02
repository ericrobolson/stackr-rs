/// Definitions for addressing words.
mod addressing;
/// Definitions for compiling words.
mod compiler;
/// Definitions for conditional words.
mod conditionals;
/// Definitions for control flow words.
mod control_flow;
/// Definitions for equality words.
mod equality;
/// Definitions for logic words.
mod logic;
/// Definitions for math words.
mod math;
/// Definitions for read mode words.
mod read_mode;
/// Definitions for runtime words.
mod runtime;
/// Definitions for stack words.
mod stack_ops;
/// Definitions for string words.
mod string;

use super::*;

/// A type alias for the operation a built-in word performs.
pub type BuiltIn<State> = fn(&mut Interpreter<State>) -> Result<(), Err>;

impl<State> Interpreter<State> {
    /// Register all built-in words.
    pub(crate) fn register_builtins(&mut self) {
        runtime::register_builtins(self);
        read_mode::register_builtins(self);
        compiler::register_builtins(self);
        addressing::register_builtins(self);
        stack_ops::register_builtins(self);
        equality::register_builtins(self);
        logic::register_builtins(self);
        conditionals::register_builtins(self);
        control_flow::register_builtins(self);
        math::register_builtins(self);
        string::register_builtins(self);
    }
}
