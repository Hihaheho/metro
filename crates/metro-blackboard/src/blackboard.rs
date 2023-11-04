use crate::{
    entity::{Entity, EntityId},
    entity_traits::EntityEnum,
    map::Map,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Blackboard<T: EntityEnum> {
    data: Map<EntityId<T::TypeTag>, Entity<T>>,
}
