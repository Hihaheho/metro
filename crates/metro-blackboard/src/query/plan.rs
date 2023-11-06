use crate::entity_traits::EntityEnum;

use super::dyn_query::DynQueryId;

pub struct QueryPlan<Enum: EntityEnum> {
    phases: Vec<QueryPhase<Enum>>,
}

pub struct QueryPhase<Enum: EntityEnum> {
    tasks: Vec<QueryTask<Enum>>,
}

pub struct QueryTask<Enum: EntityEnum> {
    query_id: DynQueryId<Enum>,
}

/// These must be implemented manually not to require additional bound like `Enum: Default`.
mod default {
    use super::*;
    impl<Enum: EntityEnum> Default for QueryPhase<Enum> {
        fn default() -> Self {
            Self {
                tasks: Default::default(),
            }
        }
    }

    impl<Enum: EntityEnum> Default for QueryPlan<Enum> {
        fn default() -> Self {
            Self {
                phases: Default::default(),
            }
        }
    }
}
