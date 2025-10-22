//! This is the core of anon, providing common structs, traits and functions.

pub mod buffered_iter;
pub mod context;
pub mod diagnostic;
pub mod interner;
pub mod line_map;
pub mod location;
pub mod source;
pub mod span;

pub use crate::source::SourceMap;
