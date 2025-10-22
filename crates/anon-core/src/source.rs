use std::path::PathBuf;

use crate::context::Contextual;

pub type SourceIdIndex = u32;

/// Source ID to identify a source in the source map
#[derive(Debug, Clone, Copy)]
pub struct SourceId(SourceIdIndex);

pub type WithID<Val> = Contextual<SourceId, Val>;

/// single Source with path and content
#[derive(Debug)]
pub struct Source {
    pub file_name: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct SourceMap {
    sources: Vec<Source>,
}

impl Default for SourceMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_or_get(&mut self, source: Source) -> SourceId {
        let id = self.sources.len();
        self.sources.push(source);

        SourceId(id as u32)
    }

    pub fn get(&self, source_id: SourceId) -> &Source {
        debug_assert!(
            (source_id.0 as usize) < self.sources.len(),
            "Internal Error: Invalid source id"
        );
        unsafe { self.sources.get_unchecked(source_id.0 as usize) }
    }

    pub fn get_content(&self, source_id: SourceId) -> &str {
        &self.get(source_id).content
    }
}
