use crate::{
    component::{error::ComponentResult, Component, Components},
    entity::{builder::EntityBuilder, entities::Entities, error::EntityResult, Entity},
    query::{query_data::QueryData, Query},
    resource::{error::ResourceResult, Resource, Resources},
};

#[derive(Default)]
pub struct World {
    pub(crate) entities: Entities,
    pub(crate) components: Components,
    pub(crate) resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    pub fn kill(&mut self, entity: Entity) -> EntityResult {
        self.entities.kill(entity)?;
        self.components.remove_entity(entity);
        Ok(())
    }

    pub fn insert_component<C: Component>(
        &mut self,
        entity: Entity,
        component: C,
    ) -> ComponentResult {
        self.components.insert(entity, component)
    }

    pub fn register_component<C: Component>(&mut self) {
        self.components.register::<C>()
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) -> Option<C> {
        self.components.remove(entity)
    }

    pub fn create_entity(&mut self) -> Entity {
        let (entity, reused) = self.entities.spawn();

        if !reused {
            self.components.expand();
        }

        entity
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> ResourceResult {
        self.resources.insert(resource)
    }

    pub fn get_resource<R: Resource>(&self) -> ResourceResult<&R> {
        self.resources.get()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> ResourceResult<&mut R> {
        self.resources.get_mut()
    }

    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove()
    }

    pub fn query<'q, D: QueryData<'q>>(&'q mut self) -> Query<'q, D> {
        Query::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        query::{fetch::Fetch, res::Res, Query},
        test_commons::{Health, Mana, Motd},
    };

    use super::World;

    #[test]
    fn should_query_for_one_component() {
        let mut world = World::new();

        world.spawn().with_component(Health(100)).build();
        world.spawn().with_component(Health(200)).build();
        world.spawn().with_component(Health(300)).build();
        world.spawn().with_component(Health(400)).build();
        world.spawn().with_component(Health(500)).build();
        world.spawn().with_component(Health(600)).build();
        world.spawn().with_component(Health(700)).build();
        world.spawn().with_component(Health(800)).build();

        let query: Query<Fetch<Health>> = world.query::<Fetch<Health>>();
        // TODO: Make it see the type!
        // let query = world.query::<Fetch<Health>>();

        for health in query {
            eprintln!("{health:?}")
        }
    }

    #[test]
    fn should_query_for_two_components() {
        let mut world = World::new();

        world
            .spawn()
            .with_component(Health(100))
            .with_component(Mana(300))
            .build();
        world
            .spawn()
            .with_component(Health(200))
            .with_component(Mana(400))
            .build();
        world
            .spawn()
            .with_component(Health(300))
            .with_component(Mana(500))
            .build();
        world
            .spawn()
            .with_component(Health(400))
            .with_component(Mana(600))
            .build();

        let query: Query<(Fetch<Health>, Fetch<Mana>)> =
            world.query::<(Fetch<Health>, Fetch<Mana>)>();

        for (health, mana) in query {
            eprintln!("({health:?}, {mana:?})");
        }
    }

    #[test]
    fn should_query_for_resource() {
        let mut world = World::new();

        world.spawn().with_component(Health(100)).build();

        world.insert_resource(Motd("Hello, World!")).unwrap();

        let query: Query<(Fetch<Health>, Res<Motd>)> = world.query();

        for (health, motd) in query {
            eprintln!("({health:?}, {motd:?})");
        }

        panic!()
    }
}
