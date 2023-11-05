pub trait TypeTag {
    fn type_info(&self) -> TypeInfo;
}

pub struct TypeInfo {
    pub type_id: std::any::TypeId,
    pub name: &'static str,
}

pub trait EntityEnum: Sized {
    type TypeTag: TypeTag + IdBounds;
    type EntityId: IdBounds;
    fn type_tag(&self) -> Self::TypeTag;
    fn list_type_tags() -> &'static [Self::TypeTag];
    fn type_tag_of<T: IntoEnum<Self>>() -> Self::TypeTag;
}

pub trait IntoEnum<T: EntityEnum> {
    fn into_enum(self) -> T;
}

pub trait FromEntity<T>: EntityEnum {
    fn from_entity(entity: T) -> Self;
}

pub trait EnumDowncast<T>: FromEntity<T> {
    fn enum_downcast(self) -> T;
    fn enum_downcast_ref(&self) -> &T;
    fn enum_downcast_mut(&mut self) -> &mut T;
}

impl<T, Enum: FromEntity<T>> IntoEnum<Enum> for T {
    fn into_enum(self) -> Enum {
        Enum::from_entity(self)
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
