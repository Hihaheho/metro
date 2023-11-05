use crate::entity_traits::EntityEnum;

use super::dyn_query::DynQueryId;

#[derive(Default)]
pub struct QueryPlan<Enum: EntityEnum> {
    phases: Vec<QueryPhase<Enum>>,
}

#[derive(Default)]
pub struct QueryPhase<Enum: EntityEnum> {
    tasks: Vec<QueryTask<Enum>>,
}

pub struct QueryTask<Enum: EntityEnum> {
    query_id: DynQueryId<Enum>,
}
