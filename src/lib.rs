pub mod system;
pub mod world;
pub mod resource;
use derive_more::derive::{Deref, DerefMut};
pub use world::World;
type ChangeTick = u32;
pub struct PlayerComponent;
#[derive(Deref, DerefMut, Default)]
pub struct SavedQuery<Q: hecs::Query>(pub hecs::PreparedQuery<Q>);
unsafe impl<Q: hecs::Query> Send for SavedQuery<Q>{}
unsafe impl<Q: hecs::Query> Sync for SavedQuery<Q>{}


#[cfg(test)]
mod tests {
    use bevy_utils::tracing::debug;
    use system::{functionsystem::FunctionSystem, IntoSystem, System};

    use super::*;

    struct C1(u8);

    #[test]
    fn it_works() {
        let mut w = World::new();
        let _ = w.spawn((C1(9),));
        let mut s1 = IntoSystem::into_system(system1);
        s1.run((), &mut w);
    }

    fn system2() {
    }
    fn system1(mut q: hecs::PreparedQueryBorrow<(&C1,)>) {
        let (_, (c,)) = q.iter().next().unwrap();
        assert_eq!(9, c.0);
    }
}


