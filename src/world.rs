use derive_more::derive::{Deref, DerefMut};
use hecs::Entity;

use crate::{resource::ResourceComponent, ChangeTick, PlayerComponent};

#[derive(Deref, DerefMut)]
pub struct World {
    #[deref] #[deref_mut]
    pub(crate) hworld: hecs::World,
    change_tick: ChangeTick,
    resource_entity: Entity,
    player_entity: Entity,
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
        let player_entity = hworld.spawn((PlayerComponent,));
        return Self {
            hworld, resource_entity, player_entity,
            change_tick: 1,
        };
    }
    pub fn resource_entity(&self) -> Entity { self.resource_entity }
    pub fn player_entity(&self) -> Entity { self.player_entity }
}
