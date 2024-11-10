use derive_more::derive::{Deref, DerefMut};
use hecs::Entity;

use crate::{resource::{Resource, ResourceComponent}, ChangeTick};

#[derive(Deref, DerefMut)]
pub struct World {
    #[deref] #[deref_mut]
    pub(crate) hworld: hecs::World,
    change_tick: ChangeTick,
    resource_entity: Entity,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub(crate) fn increment_change_tick(&mut self) -> u32 {
        self.change_tick += 1;
        return self.change_tick;
    }
    #[must_use]
    pub fn new() -> Self {
        let mut hworld = hecs::World::new();
        let resource_entity = hworld.spawn((ResourceComponent,));
        return Self {
            hworld, resource_entity, 
            change_tick: 1,
        };
    }
    pub fn resource_entity(&self) -> Entity { self.resource_entity }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.hworld.insert_one(self.resource_entity, resource).unwrap();
    }
    pub fn get_resource<R: Resource>(&mut self) -> hecs::Ref<'_, R> {
        self.hworld.get::<&R>(self.resource_entity).unwrap()
    }
}
