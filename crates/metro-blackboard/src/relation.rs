use crate::{entity::EntityId, entity_traits::EntityEnum};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Relation<T: EntityEnum> {
    from: EntityId<T::TypeTag>,
    to: EntityId<T::TypeTag>,
}
