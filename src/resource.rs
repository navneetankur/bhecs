use hecs::{Component, PreparedQuery, PreparedQueryBorrow, PreparedQueryIter, PreparedView};

use crate::{system::systemparam::SystemParam, World};
pub trait Resource: Component {}
pub struct ResourceComponent;
impl<T: Component> Resource for T {}

pub struct ResourceParamState<Q: hecs::Query + 'static> {
    query: PreparedQuery<Q>,
    borrow: Option<Borrow<Q>>,
}
struct Borrow<Q: hecs::Query + 'static> {
    iter: PreparedQueryIter<'static, Q>,
    query_borrow: PreparedQueryBorrow<'static, Q>,
}
unsafe impl<Q: hecs::Query> Send for ResourceParamState<Q>{}
unsafe impl<Q: hecs::Query> Sync for ResourceParamState<Q>{}
impl<Q: hecs::Query> Default for ResourceParamState<Q> {
    fn default() -> Self {
        Self { query: Default::default(), borrow: Default::default() }
    }
}
impl<Q: hecs::Query> ResourceParamState<Q> {
    pub fn lock<'w>(&mut self, world: &'w World) -> Option<<Q as hecs::Query>::Item<'w>> {
        use core::mem::transmute;
        let mut query_borrow = self.query.query(world);
        let iter = query_borrow.iter();
        // # Safety
        // transmut changes lifetimes. So ensure lifetime gurantees manually.
        // resource is borrowed for world. So no dangling pointer.
        // lock by hecs on this column, will be gone when query_borrow drops.
        // self.unlock() will drop it. When called from system.apply.
        let iter = unsafe { transmute::<hecs::PreparedQueryIter<'_, Q>, hecs::PreparedQueryIter<'_, Q>>(iter) };
        let query_borrow = unsafe { transmute::<hecs::PreparedQueryBorrow<'_, Q>, hecs::PreparedQueryBorrow<'_, Q>>(query_borrow) };
        let borrow = Borrow::<Q> { iter, query_borrow };
        self.borrow = Some(borrow);
        let rv =  self.borrow.as_mut().unwrap().iter.next().map(|v| v.1);
        // # Safety. Borrow is actually from world too. It won't live past function call.
        // unlock called by system.apply will remove hecs lock.
        return unsafe { transmute::<std::option::Option<<Q as hecs::Query>::Item<'_>>, std::option::Option<<Q as hecs::Query>::Item<'_>>>(rv) };
    }
    pub fn unlock(&mut self) {
        self.borrow = None;
    }
}
impl<R: Resource> SystemParam for &R {
    type State = ResourceParamState<&'static R>;

    type Item<'world, 'state> = &'world R;

    fn init_state(_: &mut crate::World, _: &mut crate::system::systemmeta::SystemMeta) -> Self::State {
        Default::default()
    }

    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _: &crate::system::systemmeta::SystemMeta,
        world: &'world crate::World,
        _: crate::ChangeTick,
    ) -> Self::Item<'world, 'state> {
        let a = state.lock(world);
        return a.expect("Resource doesn't exist");
    }

    fn apply(state: &mut Self::State, _: &crate::system::systemmeta::SystemMeta, _: &mut crate::World) {
        state.unlock();
    }
}
impl<R: Resource> SystemParam for &mut R {
    type State = ResourceParamState<&'static mut R>;

    type Item<'world, 'state> = &'world mut R;

    fn init_state(_: &mut crate::World, _: &mut crate::system::systemmeta::SystemMeta) -> Self::State {
        Default::default()
    }

    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _: &crate::system::systemmeta::SystemMeta,
        world: &'world crate::World,
        _: crate::ChangeTick,
    ) -> Self::Item<'world, 'state> {
        let a = state.lock(world);
        return a.unwrap();
    }

    fn apply(state: &mut Self::State, _: &crate::system::systemmeta::SystemMeta, _: &mut crate::World) {
        state.unlock();
    }
}


#[cfg(test)]
mod tests {
    use crate::system::{systeminput::In, IntoSystem, System};

    use super::*;

    struct R1(u8);

    #[test]
    fn resource_reference() {
        let mut w = World::new();
        w.insert_resource(R1(7));
        let mut s1 = IntoSystem::into_system(get_r1);
        let rv = s1.run((), &mut w);
        assert_eq!(rv, 7);
    }
    #[test]
    fn run_twice() {
        let mut w = World::new();
        w.insert_resource(R1(7));
        let mut s1 = IntoSystem::into_system(get_r1);
        let rv = s1.run((), &mut w);
        assert_eq!(rv, 7);
        let mut s2 = IntoSystem::into_system(get_r1);
        let _ = s2.run((), &mut w);
    }
    fn get_r1(r1: &R1) -> u8 { r1.0 }

    fn resource_mut() {
        let mut w = World::new();
        w.insert_resource(R1(0));
        let mut s1 = IntoSystem::into_system(set_r1);
        s1.run(3, &mut w);
        let r1 = w.get_resource::<R1>();
        assert_eq!(3, r1.0);
    }
    #[allow(clippy::needless_pass_by_value)]
    fn set_r1(v: In<u8>, r1: &mut R1) {
        r1.0 = v.0
    }
    // this panics and crashes
    // #[test]
    // #[should_panic]
    fn panic_on_simul_borrow() {
        let mut s1 = IntoSystem::into_system(simul_system);
        let mut w = World::new();
        w.insert_resource(R1(4));
        s1.run((), &mut w);
    }
    fn simul_system(_: &R1, _: &mut R1){}
}
