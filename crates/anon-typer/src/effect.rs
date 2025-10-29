#[derive(Clone, Debug, PartialEq)]

pub enum Effect {
    Pure,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EffectRow {
    pub effect: Effect,
}
