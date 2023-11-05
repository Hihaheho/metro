use crate::{entity_traits::EntityEnum, map::Map};

use super::dyn_query::DynQueryId;

pub struct RawResult<Enum: EntityEnum> {
    pub map: Map<DynQueryId<Enum>, Vec<Enum>>,
}
