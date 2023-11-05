use std::sync::atomic::{AtomicU32, Ordering};

use crate::entity_traits::{EntityEnum, IntoEnum};

use super::dyn_query::{DynQuery, DynQueryId};

pub struct QueryDispatcher<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
    next_id: AtomicU32,
    sender: std::sync::mpsc::Sender<(DynQueryId<Enum>, DynQuery<Enum>)>,
}

impl<Enum: EntityEnum> QueryDispatcher<Enum> {
    pub fn generate_query_id<T: IntoEnum<Enum>>(&self) -> DynQueryId<Enum> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        DynQueryId {
            id,
            type_tag: Enum::type_tag_of::<T>(),
        }
    }
}
