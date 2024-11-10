pub mod event;
pub mod system;
pub mod world;
pub mod resource;
pub use world::World;
pub use hecs::PreparedView as View;
pub use hecs::PreparedQueryIter as Query;
use derive_more::derive::{Deref, DerefMut};
type ChangeTick = u32;


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
    fn system1(mut q: hecs::PreparedQueryIter<(&C1,)>) {
        let (_, (c,)) = q.next().unwrap();
        assert_eq!(9, c.0);
    }
}


