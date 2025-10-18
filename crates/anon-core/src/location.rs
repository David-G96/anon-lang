pub type LocationIndex = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    line: LocationIndex,
    column: LocationIndex,
}

impl Location {
    pub fn new(
        line: impl Into<LocationIndex>,
        column: impl Into<LocationIndex>,
    ) -> Self {
        Self {
            line: line.into(),
            column: column.into(),
        }
    }

    pub fn line(self) -> LocationIndex {
        self.line
    }

    pub fn column(self) -> LocationIndex {
        self.column
    }
}
