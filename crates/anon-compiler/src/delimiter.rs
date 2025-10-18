#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Delimiter {
    /// (
    LParen,
    /// )
    RParen,
    /// ,
    Comma,
    /// ::
    Annotate,
    /// _
    UnderScore,
}
