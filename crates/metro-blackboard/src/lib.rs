pub mod blackboard;
pub mod commands;
pub mod entity;
pub mod entity_traits;
pub mod map;
pub mod query;
pub mod relation;

pub mod prelude {
    pub use crate::derive::*;
    pub use crate::entity_traits::*;
}

pub use crate::derive::*;

mod derive {
    #[cfg(feature = "derive")]
    pub use metro_macros::*;
}
