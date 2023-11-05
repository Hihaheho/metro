use uuid::Uuid;

use crate::{
    commands::{Command, Commands},
    entity::{EntityEntry, EntityId},
    entity_traits::{EntityEnum, FromEntity, TypeTag},
    map::Map,
    relation::Relation,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Blackboard<Enum: EntityEnum> {
    data: Map<EntityId<Enum::TypeTag>, EntityEntry<Enum>>,
}

impl<Enum: EntityEnum> Default for Blackboard<Enum> {
    fn default() -> Self {
        Self {
            data: Map::default(),
        }
    }
}

impl<Enum: EntityEnum> Blackboard<Enum> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Enum, Tag> Blackboard<Enum>
where
    Tag: TypeTag + Copy + Clone + Eq + Ord + std::hash::Hash + std::fmt::Debug,
    Enum: EntityEnum<TypeTag = Tag>,
{
    pub fn apply_commands(&mut self, commands: Commands<Enum>) {
        for command in commands.into_iter() {
            match command {
                Command::Insert {
                    id,
                    relation,
                    entity,
                } => self.handle_insert(id, relation, entity),
                Command::Delete { id } => self.handle_delete(id),
                Command::Update { id, entity } => self.handle_update(id, entity),
                Command::UpdateRelation { id, relation } => {
                    self.handle_update_relation(id, relation)
                }
            }
        }
    }

    fn handle_insert(&mut self, id: EntityId<Tag>, relation: Option<Relation<Enum>>, entity: Enum) {
        let entry = EntityEntry::new(entity, relation);
        self.data.insert(id, entry);
    }

    fn handle_delete(&mut self, id: EntityId<Tag>) {
        todo!()
    }

    fn handle_update(&mut self, id: EntityId<Tag>, entity: Enum) {
        todo!()
    }

    fn handle_update_relation(&mut self, id: EntityId<Tag>, relation: Relation<Enum>) {
        todo!()
    }

    /// Remove relation from entity, and update graph
    fn remove_relation(&mut self, id: EntityId<Tag>) {}

    /// Add relation to entity, and update graph
    fn add_relation(&mut self, id: EntityId<Tag>, relation: Relation<Enum>) {}
}
