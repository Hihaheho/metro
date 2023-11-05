use uuid::Uuid;

use crate::{
    entity::EntityId,
    entity_traits::{EntityEnum, FromEntity, IntoEnum},
    relation::Relation,
};

pub struct Commands<Enum: EntityEnum> {
    commands: Vec<Command<Enum>>,
}

impl<Enum: EntityEnum> Default for Commands<Enum> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Enum: EntityEnum> Commands<Enum> {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn insert<T: IntoEnum<Enum>>(&mut self, uuid: Uuid, entity: T) -> &mut Self {
        let entity = entity.into_enum();
        let id = EntityId {
            type_tag: entity.get_type_tag(),
            uuid,
        };
        self.commands.push(Command::Insert {
            id,
            relation: None,
            entity,
        });
        self
    }

    pub fn insert_with_relation<T: IntoEnum<Enum>>(
        &mut self,
        uuid: Uuid,
        entity: T,
        relation: Relation<Enum>,
    ) -> &mut Self {
        let entity = entity.into_enum();
        let id = EntityId {
            type_tag: entity.get_type_tag(),
            uuid,
        };
        self.commands.push(Command::Insert {
            id,
            relation: Some(relation),
            entity,
        });
        self
    }

    pub fn delete(&mut self, id: EntityId<Enum::TypeTag>) -> &mut Self {
        self.commands.push(Command::Delete { id });
        self
    }

    pub fn update<T>(&mut self, id: EntityId<Enum::TypeTag>, entity: T) -> &mut Self
    where
        Enum: FromEntity<T>,
    {
        let entity = Enum::from_entity(entity);
        self.commands.push(Command::Update { id, entity });
        self
    }

    pub fn update_relation(
        &mut self,
        id: EntityId<Enum::TypeTag>,
        relation: Relation<Enum>,
    ) -> &mut Self {
        self.commands.push(Command::UpdateRelation { id, relation });
        self
    }
}

impl<Enum: EntityEnum> IntoIterator for Commands<Enum> {
    type Item = Command<Enum>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}

pub enum Command<Enum: EntityEnum> {
    Insert {
        id: EntityId<Enum::TypeTag>,
        relation: Option<Relation<Enum>>,
        entity: Enum,
    },
    Delete {
        id: EntityId<Enum::TypeTag>,
    },
    Update {
        id: EntityId<Enum::TypeTag>,
        entity: Enum,
    },
    UpdateRelation {
        id: EntityId<Enum::TypeTag>,
        relation: Relation<Enum>,
    },
}
