
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let,
    In,
    If,
    Then,
    Else,
    Match,
    Case,
    Import,
    Export,
    Class,
    Instance,

    Data,
    Type,
}
