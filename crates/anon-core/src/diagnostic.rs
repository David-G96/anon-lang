use std::{borrow::Cow, fmt::Debug};

use crate::span::Span;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Severity {
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

#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub span: Span,
    pub replacement: String,
    pub message: Option<String>,
}

/// 抽象 diagnostic 接口（对外）
pub trait Diagnostic: Send + Sync + Debug {
    fn severity(&self) -> Severity;
    fn message(&self) -> Cow<'_, str>; // human readable
    fn code(&self) -> Option<&str>; // machine readable code like "E042"
    fn labels(&self) -> Vec<Label>; // owned Vec 便于对象安全
    fn notes(&self) -> Vec<Cow<'_, str>>;
    fn suggestions(&self) -> Vec<Suggestion>;
    // 如果你需要克隆 Box<dyn Diagnostic>，可以在 trait 中加入 clone_box（见下）
    fn clone_box(&self) -> Box<dyn Diagnostic>;
}

#[derive(Debug, Clone)]
pub struct SimpleDiagnostic {
    pub severity: Severity,
    pub message: String,
    pub primary_span: Span,
    pub children: Option<Box<Self>>,
}

impl SimpleDiagnostic {
    pub fn new(
        level: Severity,
        primary_message: String,
        primary_span: Span,
        children: Option<Box<Self>>,
    ) -> Self {
        Self {
            severity: level,
            message: primary_message,
            primary_span,
            children,
        }
    }
}

// impl Diagnostic for SimpleDiagnostic {
//     fn severity(&self) -> Severity {
//         self.severity.clone()
//     }
//     fn message(&self) -> Cow<'_, str> {
//         Cow::Borrowed(&self.message)
//     }
//     // fn code(&self) -> Option<&str> {
//     //     self.code.as_deref()
//     // }
//     // fn labels(&self) -> Vec<Label> {
//     //     self.labels.clone()
//     // }
//     // fn notes(&self) -> Vec<Cow<'_, str>> {
//     //     self.notes.iter().map(|s| Cow::Owned(s.clone())).collect()
//     // }
//     // fn suggestions(&self) -> Vec<Suggestion> {
//     //     self.suggestions.clone()
//     // }
//     fn clone_box(&self) -> Box<dyn Diagnostic> {
//         Box::new(self.clone())
//     }
// }

impl std::fmt::Display for SimpleDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SimpleDiagnostic {}

#[derive(Debug)]
pub struct DiagnosticCollector {
    bag: Vec<Box<dyn Diagnostic>>,
}
