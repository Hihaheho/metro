pub trait TypeTag {
    const ANY: Self;
    fn type_info(&self) -> TypeInfo;
}

pub struct TypeInfo {
    pub type_id: std::any::TypeId,

    /// The name of the tag, not the type.
    ///
    /// For example, if the enum is `Entity::Apple(TheRedFruit)`, the tag name is `Apple` not
    /// `TheRedFruit`.
    pub tag_name: &'static str,
}

pub trait EntityEnum: Sized + 'static {
    const TYPE_TAGS: &'static [Self::TypeTag];
    type TypeTag: TypeTag + IdBounds;
    type EntityId: IdBounds;
    fn type_tag(&self) -> Self::TypeTag;
    fn type_tag_of<T>() -> Self::TypeTag
    where
        Self: FromEntity<T>;
}

pub trait IntoEnum<T> {
    fn into_enum(self) -> T;
}

// TODO: More nice name?
pub trait FromEntity<T>: EntityEnum {
    fn from_entity(entity: T) -> Self;
}

impl<T, Enum: FromEntity<T>> IntoEnum<Enum> for T {
    fn into_enum(self) -> Enum {
        Enum::from_entity(self)
    }
}

/// EnumEntity itself can be used as like an entity.
impl<T: EntityEnum> FromEntity<T> for T {
    fn from_entity(entity: T) -> Self {
        entity
    }
}

#[cfg(feature = "serde")]
mod bounds {
    pub trait IdBounds:
        Copy
        + Clone
        + Eq
        + Ord
        + std::hash::Hash
        + std::fmt::Debug
        + serde::Serialize
        + serde::de::DeserializeOwned
    {
    }
    impl<T> IdBounds for T where
        T: Copy
            + Clone
            + Eq
            + Ord
            + std::hash::Hash
            + std::fmt::Debug
            + serde::Serialize
            + serde::de::DeserializeOwned
    {
    }
}
#[cfg(not(feature = "serde"))]
mod bounds {
    pub trait IdBounds: Copy + Clone + Eq + Ord + std::hash::Hash + std::fmt::Debug {}
    impl<T> IdBounds for T where T: Copy + Clone + Eq + Ord + std::hash::Hash + std::fmt::Debug {}
}

use bounds::IdBounds;
