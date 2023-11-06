use metro_blackboard::{blackboard::Blackboard, entity_traits::EntityEnum};

pub struct Agent<Enum: EntityEnum> {
    blackboard: Blackboard<Enum>,
}
