use metro_blackboard::entity_traits::EntityEnum;

pub trait RuntimeHandle<Enum: EntityEnum> {
    fn spawn(&self, unit_id: Enum::EntityId);
}
