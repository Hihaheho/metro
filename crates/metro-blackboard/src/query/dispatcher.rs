use std::sync::atomic::{AtomicU32, Ordering};

use crate::entity_traits::{EntityEnum, MemberMarker};

use super::dyn_query::{DynQuery, DynQueryId};

pub struct QueryDispatcher<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
    next_id: AtomicU32,
    sender: std::sync::mpsc::Sender<(DynQueryId<Enum>, DynQuery<Enum>)>,
}

impl<Enum: EntityEnum> QueryDispatcher<Enum> {
    pub(crate) fn new(sender: std::sync::mpsc::Sender<(DynQueryId<Enum>, DynQuery<Enum>)>) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            next_id: AtomicU32::new(0),
            sender,
        }
    }
    pub(crate) fn generate_query_id<T>(&self) -> DynQueryId<Enum>
    where
        Enum: MemberMarker<T>,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        DynQueryId {
            id,
            type_tag: Enum::type_tag_of::<T>(),
        }
    }
}
