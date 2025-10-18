#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// =
    Eq,
    /// -
    Negate,
    /// ->
    Arrow,
    Add,
    Div,
    Mul,
}
