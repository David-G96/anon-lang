/// puts Val in the context of Ctx
pub struct Contextual<Ctx, Val> {
    pub context: Ctx,
    pub value: Val,
}
