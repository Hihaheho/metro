pub mod runtime;

use metro_blackboard::entity_traits::{EntityEnum, IntoEnum};

use crate::surface::AgentSurface;

use self::runtime::RuntimeHandle;

/// The interface for an agent unit.
///
/// The type of this unit must be included in the EnumEntity to be registered in a blackboard.
pub trait AgentUnit: IntoEnum<Self::Target> {
    /// The type of the entity that this agent unit interacts with.
    type Target: EntityEnum;

    /// The type of the world that this agent unit interacts with.
    ///
    /// Typically, this is a pair of a world reference (or snapshot with static lifetime)
    /// and an effect dispatcher.
    type Environment;

    /// Invoke a single step of the this unit's functionality.
    ///
    /// As `RuntimeHandle` is a generic type instead of this trait's associated type,
    /// a unit is independent of a specific runtime.
    async fn invoke<R: RuntimeHandle<Self::Target>>(
        &self,
        surface: AgentSurface<Self::Target, Self::Environment, R>,
    );
}
