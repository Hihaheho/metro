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

pub trait FromEntity<T, Tag>: EntityEnum<TypeTag = Tag> {
    fn from_entity(data: T) -> Self;
    fn type_tag() -> Tag;
}

pub trait EnumDowncast<T, Tag>: FromEntity<T, Tag> {
    fn enum_downcast(self) -> T;
    fn enum_downcast_ref(&self) -> &T;
    fn enum_downcast_mut(&mut self) -> &mut T;
}
