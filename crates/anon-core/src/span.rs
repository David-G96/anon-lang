use crate::context::Contextual;

pub type SpanIndex = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: SpanIndex,
    end: SpanIndex,
}

impl Span {
    pub unsafe fn new_unchecked(
        start: impl Into<SpanIndex>,
        end: impl Into<SpanIndex>,
    ) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn new(start: impl Into<SpanIndex>, end: impl Into<SpanIndex>) -> Option<Self> {
        let start = start.into();
        let end = end.into();
        (start <= end).then(|| Self { start, end })
    }

    pub fn start(self) -> SpanIndex {
        self.start
    }

    pub fn end(self) -> SpanIndex {
        self.end
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

pub type Spanned<Val> = Contextual<Span, Val>;
