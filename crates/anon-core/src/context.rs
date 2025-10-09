use crate::interner::Interner;

pub struct Contextual<Ctx, Val> {
    pub context: Ctx,
    pub value: Val,
}

pub struct Context {
    pub interner: Interner,
}
