use crate::entity_traits::{EntityEnum, IntoEnum, TypeTag};

pub trait Query<Enum: EntityEnum> {
    type Output;

    /// Build dynamic query
    fn build(&self) -> QueryBuilder<Enum>;
    /// Collect results from dynamic results
    fn collect(&self, result: QueryResult<Enum>) -> Self::Output;
}

pub struct QueryBuilder<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
}

impl Default for QueryBuilder<Enum> {
    fn default() -> Self {
        QueryBuilder {
            _phantom: Default::default(),
        }
    }
}

pub struct QueryResult<Enum: EntityEnum> {
    _phantom: std::marker::PhantomData<Enum>,
}

impl<Enum: EntityEnum> QueryBuilder<Enum> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T: IntoEnum<Enum>, Enum: EntityEnum> Query<Enum> for T {
    type Output = Vec<&T>;

    fn build(&self) -> QueryBuilder<Enum> {
        QueryBuilder {
            _phantom: std::marker::PhantomData,
        }
    }

    fn collect(&self, result: QueryResult<Enum>) -> Self::Output {}
}
