use super::*;

#[derive(Debug, Clone)]
pub(crate) enum RamValue<State> {
    BuiltIn(BuiltIn<State>),
    Value(Value),
    Address(Address),
    Compiled(Vec<Instruction>),
}
impl<State> From<StackValue> for RamValue<State> {
    fn from(value: StackValue) -> Self {
        match value {
            StackValue::Address(address) => RamValue::Address(address),
            StackValue::Value(value) => RamValue::Value(value),
        }
    }
}
impl<State> RamValue<State> {
    pub fn address(&self) -> Option<Address> {
        match self {
            RamValue::Address(address) => Some(*address),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn value(&self) -> Option<Value> {
        match self {
            RamValue::Value(value) => Some(value.clone()),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn expect_address(&self) -> Address {
        match self {
            RamValue::Address(address) => *address,
            _ => panic!("Expected an address"),
        }
    }

    #[allow(unused)]
    pub fn expect_value(&self) -> Value {
        match self {
            RamValue::Value(value) => value.clone(),
            _ => panic!("Expected a value"),
        }
    }
}
