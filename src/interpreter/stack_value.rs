use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    Address(Address),
    Value(Value),
}
impl From<f32> for StackValue {
    fn from(value: f32) -> Self {
        StackValue::Value(Value::Number(value))
    }
}
impl From<Address> for StackValue {
    fn from(value: Address) -> Self {
        StackValue::Address(value)
    }
}

impl From<&Address> for StackValue {
    fn from(value: &Address) -> Self {
        StackValue::Address(value.clone())
    }
}
impl From<Value> for StackValue {
    fn from(value: Value) -> Self {
        StackValue::Value(value)
    }
}
