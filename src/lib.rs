pub mod system;
pub mod world;
pub use world::World;
type ChangeTick = u32;


#[cfg(test)]
mod tests {
    use system::{functionsystem::FunctionSystem, IntoSystem, System};

    use super::*;

    struct C1(u8);

    #[test]
    fn it_works() {
        let mut w = World::new();
        let e = w.spawn((C1(9),));
        let mut s1 = IntoSystem::into_system(system1);
        s1.initialize(&mut w);
        unsafe { s1.run((), &mut w) };
        // let s1 = FunctionSystem { func: system1, param_state: None, system_meta: Default::default(),
        // marker: core::marker::PhantomData
        // };
        // let s2 = IntoSystem::into_system(system2);
    }

    fn system2() {
    }
    fn system1(mut q: hecs::PreparedQueryBorrow<(&C1,)>) {
        let (e, (c,)) = q.iter().next().unwrap();
        println!("{}", c.0);
    }
}


