use uuid::Uuid;

use crate::{
    entity_traits::{EntityEnum, TypeTag},
    map::Set,
    relation::Relation,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityId<T: TypeTag> {
    pub(crate) type_tag: T,
    pub(crate) uuid: Uuid,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityEntry<T: EntityEnum> {
    pub(crate) referred_by: Set<EntityId<T::TypeTag>>,
    pub(crate) relation: Option<Relation<T>>,
    pub data: T,
}

impl<T: EntityEnum> EntityEntry<T> {
    pub(crate) fn new(data: T, relation: Option<Relation<T>>) -> Self {
        Self {
            referred_by: Default::default(),
            relation,
            data,
        }
    }
}

impl<T: EntityEnum> EntityEntry<T> {
    pub fn referred_by(&self) -> &Set<EntityId<T::TypeTag>> {
        &self.referred_by
    }

    pub fn relation(&self) -> Option<&Relation<T>> {
        self.relation.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use uuid::uuid;

    use super::*;

    #[test]
    fn primary_index_is_type_tag() {
        #[derive(
            Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
        )]
        enum TestTypeTag {
            Any,
            A,
            B,
            C,
        }

        impl TypeTag for TestTypeTag {
            const ANY: Self = Self::Any;
            fn type_info(&self) -> crate::entity_traits::TypeInfo {
                todo!()
            }
        }

        let mut ids = vec![
            EntityId {
                type_tag: TestTypeTag::B,
                uuid: uuid!("00000000-0000-0000-0000-000000000000"),
            },
            EntityId {
                type_tag: TestTypeTag::A,
                uuid: uuid!("00000000-0000-0000-0000-000000000001"),
            },
            EntityId {
                type_tag: TestTypeTag::C,
                uuid: uuid!("00000000-0000-0000-0000-000000000002"),
            },
            EntityId {
                type_tag: TestTypeTag::B,
                uuid: uuid!("00000000-0000-0000-0000-000000000003"),
            },
        ];
        ids.sort();
        assert_eq!(
            ids,
            vec![
                EntityId {
                    type_tag: TestTypeTag::A,
                    uuid: uuid!("00000000-0000-0000-0000-000000000001"),
                },
                EntityId {
                    type_tag: TestTypeTag::B,
                    uuid: uuid!("00000000-0000-0000-0000-000000000000"),
                },
                EntityId {
                    type_tag: TestTypeTag::B,
                    uuid: uuid!("00000000-0000-0000-0000-000000000003"),
                },
                EntityId {
                    type_tag: TestTypeTag::C,
                    uuid: uuid!("00000000-0000-0000-0000-000000000002"),
                },
            ]
        );
    }
}
