use bevy_ecs::prelude::*;

/// A component that represents a subject in a triple. "<Player> likes apples".
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Subject<T> {
    _phantom: std::marker::PhantomData<T>,
}

/// A component that represents a predicate in a triple. "Player <likes> apples".
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Object<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Predicate<T> {
    _phantom: std::marker::PhantomData<T>,
}
