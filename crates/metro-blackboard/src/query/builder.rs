use crate::entity_traits::{EntityEnum, IntoEnum};

use super::{dispatcher::QueryDispatcher, raw_result::RawResult};

pub trait Query<Enum: EntityEnum> {
    type Output;

    /// Build dynamic query
    fn build(&self, dispatcher: QueryDispatcher<Enum>);
    /// Collect results from dynamic results
    fn collect(&self, raw_result: RawResult<Enum>) -> Self::Output;
}

#[derive(Default)]
pub struct Select<Enum, T> {
    _phantom: std::marker::PhantomData<(Enum, T)>,
}

impl<Enum: EntityEnum, T: IntoEnum<Enum>> Select<Enum, T> {
    pub fn id(self, id: Enum::EntityId) -> SelectById<Enum, T> {
        SelectById::new(id)
    }
    pub fn ids(self, ids: Vec<Enum::EntityId>) -> SelectByIds<Enum, T> {
        SelectByIds::new(ids)
    }
}

impl<Enum: EntityEnum, T: IntoEnum<Enum>> Query<Enum> for Select<Enum, T> {
    type Output = T;

    fn build(&self, dispatcher: QueryDispatcher<Enum>) {}

    fn collect(&self, raw_result: RawResult<Enum>) -> Self::Output {
        todo!()
    }
}

pub struct SelectById<Enum: EntityEnum, T> {
    _phantom: std::marker::PhantomData<(Enum, T)>,
    id: Enum::EntityId,
}

impl<Enum: EntityEnum, T> SelectById<Enum, T> {
    pub fn new(id: Enum::EntityId) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            id,
        }
    }
}

pub struct SelectByIds<Enum: EntityEnum, T> {
    _phantom: std::marker::PhantomData<(Enum, T)>,
    id: Vec<Enum::EntityId>,
}

impl<Enum: EntityEnum, T> SelectByIds<Enum, T> {
    pub fn new(id: Vec<Enum::EntityId>) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            id,
        }
    }
}
