use amethyst::core::ecs;
use std::collections::HashMap;

/// Provides a helper for associating a callback with a given entity.
///
/// This structure is generic on the type of data given to the callback and the
/// return parameter.
pub(super) struct EntityObservers<TData, TReturn = ()> {
    observers: HashMap<ecs::Entity, fn(&mut TData, &mut ecs::World) -> TReturn>,
}

impl<TData, TReturn> EntityObservers<TData, TReturn> {
    /// Constructs a new entity observers collection.
    pub fn new() -> Self {
        Self {
            observers: HashMap::new(),
        }
    }

    /// Associates a callback with the given entity.
    pub fn add_observer(
        &mut self,
        entity: ecs::Entity,
        callback: fn(&mut TData, &mut ecs::World) -> TReturn,
    ) {
        self.observers.insert(entity, callback);
    }

    /// Removes the associated callback for the given entity.
    pub fn remove_observer(&mut self, entity: ecs::Entity) {
        self.observers.remove(&entity);
    }

    /// Gets the callback associated with the provided entity.
    ///
    /// None is returned if no callback is found.
    pub fn observer(
        &self,
        entity: ecs::Entity,
    ) -> Option<&fn(&mut TData, &mut ecs::World) -> TReturn> {
        self.observers.get(&entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use amethyst::core::ecs::world::EntitiesRes;
    use amethyst::prelude::*;

    // Builds a world that has an entities resource.
    fn build_world() -> ecs::World {
        let mut world = ecs::World::default();
        let mut sadf = EntitiesRes::default();
        world.insert(sadf);

        world
    }

    #[test]
    fn entity_observers_observer_when_no_entity_should_return_none() {
        let observers: EntityObservers<i32> = EntityObservers::new();
        let mut world = build_world();
        let entity = world.create_entity().build();

        // Attempt to get a callback that has not been previously added.
        let callback = observers.observer(entity);

        assert!(callback.is_none());
    }

    #[test]
    fn entity_observers_observer_when_contains_entity_should_return_correct_callback() {
        let mut world = build_world();
        let entity = world.create_entity().build();
        let mut expected_data = 42;
        let mut observers: EntityObservers<i32, i32> = EntityObservers::new();
        // Add a callback that simply passes through its data.
        observers.add_observer(entity, |data, _world| *data);

        // Get the callback back out and invoke it to see if it passes through its data.
        let callback = observers.observer(entity);
        let actual_data = callback.unwrap()(&mut expected_data, &mut world);

        assert_eq!(actual_data, expected_data);
    }

    #[test]
    fn entity_observers_remove_should_remove_callback() {
        let mut world = build_world();
        let entity = world.create_entity().build();
        let mut observers: EntityObservers<i32, i32> = EntityObservers::new();
        observers.add_observer(entity, |data, _world| *data);

        observers.remove_observer(entity);

        let callback = observers.observer(entity);
        assert!(callback.is_none());
    }
}
