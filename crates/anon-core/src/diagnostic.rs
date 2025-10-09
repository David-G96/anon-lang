use crate::span::Span;

#[derive(Debug)]
pub struct Diagnostic {
    pub level: Level,
    pub primary_message: String,
    pub primary_span: Span,
    pub children: Box<Self>,
}

impl Diagnostic {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Level {
    /// Error level, compiler stops at once
    Error,
    /// Warming level,
    ///  compiler might stop based on Debug/Release mode or flag
    Warning,
    /// Note level,
    Note,
    /// Help level,
    Help,
}
