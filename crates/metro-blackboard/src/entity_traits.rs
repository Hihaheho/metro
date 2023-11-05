pub trait TypeTag {
    fn type_info(&self) -> TypeInfo;
}

pub struct TypeInfo {
    pub type_id: std::any::TypeId,
    pub name: &'static str,
}

pub trait EntityEnum {
    #[cfg(not(feature = "serde"))]
    type TypeTag: TypeTag + Copy + Clone + Eq + Ord + std::hash::Hash + std::fmt::Debug;
    #[cfg(feature = "serde")]
    type TypeTag: TypeTag
        + Copy
        + Clone
        + Eq
        + Ord
        + std::hash::Hash
        + std::fmt::Debug
        + serde::Serialize
        + serde::de::DeserializeOwned;
    fn get_type_tag(&self) -> Self::TypeTag;
    fn list_type_tags() -> &'static [Self::TypeTag];
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
