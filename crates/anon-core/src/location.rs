pub type LocationIndex = u32;

/// represents a 0-indexed location in the file
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

impl<T: Into<u32>> From<(T, T)> for Location {
    fn from(value: (T, T)) -> Self {
        Self {
            line: value.0.into(),
            column: value.1.into(),
        }
    }
}
