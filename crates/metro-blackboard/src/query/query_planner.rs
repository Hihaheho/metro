use crate::{blackboard::Blackboard, entity_traits::EntityEnum};

use super::{
    dyn_query::{DynQuery, DynQueryId},
    plan::QueryPlan,
};

pub struct QueryScheduler<Enum: EntityEnum> {
    recv: std::sync::mpsc::Receiver<(DynQueryId<Enum>, DynQuery<Enum>)>,
}

impl<Enum: EntityEnum> QueryScheduler<Enum> {
    pub fn new(recv: std::sync::mpsc::Receiver<(DynQueryId<Enum>, DynQuery<Enum>)>) -> Self {
        Self { recv }
    }
    pub fn schedule(blackboard: &Blackboard<Enum>, query: DynQuery<Enum>) -> QueryPlan<Enum> {
        todo!()
    }
}
