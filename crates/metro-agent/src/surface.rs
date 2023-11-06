use metro_blackboard::{
    commands::Commands,
    entity_traits::{EntityEnum, IntoEnum},
    query::{builder::Select, dispatcher::QueryDispatcher},
};

use crate::unit::runtime::RuntimeHandle;

/// The interface to interact with the blackboard, external services, world environment, etc. from
/// AgentUnit.
pub struct AgentSurface<Enum, Env, R>
where
    Enum: EntityEnum,
    R: RuntimeHandle<Enum>,
{
    query_dispatcher: QueryDispatcher<Enum>,
    commands: Commands<Enum>,
    environment: Env,
    runtime_handle: R,
}

impl<Enum, Env, R> AgentSurface<Enum, Env, R>
where
    Enum: EntityEnum,
    R: RuntimeHandle<Enum>,
{
    pub fn new(
        query_dispatcher: QueryDispatcher<Enum>,
        environment: Env,
        runtime_handle: R,
    ) -> Self {
        Self {
            query_dispatcher,
            commands: Default::default(),
            environment,
            runtime_handle,
        }
    }

    pub fn select<T: IntoEnum<Enum>>(&self) -> Select<Enum, T> {
        Default::default()
    }
}
