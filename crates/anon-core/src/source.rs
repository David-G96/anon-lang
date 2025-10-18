use crate::context::Contextual;

pub type SourceIdIndex = u32;

#[derive(Debug, Clone, Copy)]
pub struct SourceId(SourceIdIndex);

pub type WithID<Val> = Contextual<SourceId, Val>;

#[derive(Debug)]
pub struct Source {
    pub file_name: String,
    pub content: String,
}

#[derive(Debug)]
pub struct SourceTable {
    sources: Vec<Source>,
}

impl SourceTable {
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

    pub fn get(&self, source_id: SourceId) -> Option<&Source> {
        self.sources.get(source_id.0 as usize)
    }
}
