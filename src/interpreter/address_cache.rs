use super::*;

/// Address cache. Used to remove need to lookup addresses every time.
pub(crate) struct AddressCache {
    /// Address of the read mode end.
    pub read_mode_end: Address,
    /// Address of the compile mode end.
    pub compile_end: Address,
    /// Address of the else statement.
    pub else_statement: Address,
    /// Address of the end of a statement.
    pub end_statement: Address,
    /// Address of the break statement.
    pub break_statement: Address,
    /// Address of the begin statement.
    pub begin_statement: Address,
    /// Address of the loop statement.
    pub loop_statement: Address,
}

impl AddressCache {
    /// Create a new address cache.
    pub fn uninitalized() -> Self {
        Self {
            read_mode_end: Address::default(),
            compile_end: Address::default(),
            else_statement: Address::default(),
            end_statement: Address::default(),
            break_statement: Address::default(),
            begin_statement: Address::default(),
            loop_statement: Address::default(),
        }
    }

    /// Initialize the address cache for an interpreter.
    pub fn initialize<State>(interpreter: &mut Interpreter<State>) {
        let mut cache = Self::uninitalized();

        // Cache various address for quick lookups.
        // Otherwise we would have to lookup the address of things like `]` and `;` every time.
        cache.read_mode_end = interpreter.get_address("]");
        cache.compile_end = interpreter.get_address(";");

        cache.else_statement = interpreter.get_address("else");
        cache.end_statement = interpreter.get_address("end");

        cache.break_statement = interpreter.get_address("break");
        cache.begin_statement = interpreter.get_address("begin");
        cache.loop_statement = interpreter.get_address("loop");

        interpreter.address_cache = cache;
    }
}
