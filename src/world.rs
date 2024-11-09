use derive_more::derive::{Deref, DerefMut};

use crate::ChangeTick;

#[derive(Deref, DerefMut)]
pub struct World {
    #[deref] #[deref_mut]
    pub(crate) hworld: hecs::World,
    change_tick: ChangeTick,
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
        Self {
            hworld: hecs::World::new(),
            change_tick: 1,
        }
    }
}
