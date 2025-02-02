/// Address in RAM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Address(u64);
impl Address {
    /// Create a new address.
    #[allow(unused)]
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the next address.
    pub(crate) fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
