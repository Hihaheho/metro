#[cfg(feature = "btree")]
pub type Map<K, V> = std::collections::BTreeMap<K, V>;
#[cfg(feature = "btree")]
pub type Set<T> = std::collections::BTreeSet<T>;
