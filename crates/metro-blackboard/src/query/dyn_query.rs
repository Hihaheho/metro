use crate::entity_traits::EntityEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynQueryId<Enum: EntityEnum> {
    pub type_tag: Enum::TypeTag,
    pub id: u32,
}

pub struct DynQuery<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
    /// Filter by entity id
    pub by_id: Vec<Enum::EntityId>,
    /// Filter by entity's value
    pub cond: Option<Cond<Enum>>,
    /// Filter by entity's reference
    pub referred_as_head: Option<DynQueryId<Enum>>,
    /// Filter by entity's reference
    pub referred_as_tail: Option<DynQueryId<Enum>>,
    /// Filter by entity's relation
    pub relational_cond: Option<RelationalCond<Enum>>,
}

pub struct Cond<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
    expr: DynExpr,
}

pub struct RelationalCond<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
    tail: DynQueryId<Enum>,
    head: DynQueryId<Enum>,
    expr: DynExpr,
}

pub enum DynExpr {}

/// These must be implemented manually not to require additional bound like `Enum: Default`.
mod default {
    use super::*;
    impl<Enum: EntityEnum> Default for DynQuery<Enum> {
        fn default() -> Self {
            Self {
                _phantom: Default::default(),
                by_id: Default::default(),
                cond: Default::default(),
                referred_as_head: Default::default(),
                referred_as_tail: Default::default(),
                relational_cond: Default::default(),
            }
        }
    }
}
