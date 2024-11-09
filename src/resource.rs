use hecs::{Component, PreparedQuery, PreparedQueryBorrow, PreparedView};

use crate::{system::systemparam::SystemParam, SavedQuery, World};
pub trait Resource: Component {}
pub struct ResourceComponent;
impl<T: Component> Resource for T {}

pub struct ResourceParamState<Q: hecs::Query + 'static> {
    query: PreparedQuery<Q>,
    view_borrow: Option<View<Q>>,
}
struct View<Q: hecs::Query + 'static> {
    query_borrow: PreparedQueryBorrow<'static, Q>,
    view: PreparedView<'static, Q>,
}
unsafe impl<Q: hecs::Query> Send for ResourceParamState<Q>{}
unsafe impl<Q: hecs::Query> Sync for ResourceParamState<Q>{}
impl<Q: hecs::Query> Default for ResourceParamState<Q> {
    fn default() -> Self {
        Self { query: Default::default(), view_borrow: Default::default() }
    }
}
impl<Q: hecs::Query + hecs::QueryShared> ResourceParamState<Q> {
    pub fn lock<'w>(&mut self, world: &'w World) -> Option<<Q as hecs::Query>::Item<'w>> {
        use core::mem::transmute;
        self.setup_borrows(world);
        let rv =  self.view_borrow.as_ref().unwrap().view.get(world.resource_entity());
        // # Safety. Borrow is actually from world too. It won't live past function call.
        // unlock called by system.apply will remove hecs lock.
        return unsafe { transmute::<std::option::Option<<Q as hecs::Query>::Item<'_>>, std::option::Option<<Q as hecs::Query>::Item<'_>>>(rv) };
    }
}
impl<Q: hecs::Query> ResourceParamState<Q> {
    pub fn lock_mut<'w>(&mut self, world: &'w World) -> Option<<Q as hecs::Query>::Item<'w>> {
        use core::mem::transmute;
        self.setup_borrows(world);
        let rv =  self.view_borrow.as_mut().unwrap().view.get_mut(world.resource_entity());
        // # Safety. Borrow is actually from world too. It won't live past function call.
        // unlock called by system.apply will remove hecs lock.
        return unsafe { transmute::<std::option::Option<<Q as hecs::Query>::Item<'_>>, std::option::Option<<Q as hecs::Query>::Item<'_>>>(rv) };
    }

    fn setup_borrows(&mut self, world: &World) {
        use core::mem::transmute;
        let mut query_borrow = self.query.query(world);
        let view = query_borrow.view();
        // # Safety
        // transmut changes lifetimes. So ensure lifetime gurantees manually.
        // resource is borrowed for world. So no dangling pointer.
        // lock by hecs on this column, will be gone when query_borrow drops.
        // self.unlock() will drop it. When called from system.apply.
        let view = unsafe { transmute::<hecs::PreparedView<'_, Q>, hecs::PreparedView<'_, Q>>(view) };
        let query_borrow = unsafe { transmute::<hecs::PreparedQueryBorrow<'_, Q>, hecs::PreparedQueryBorrow<'_, Q>>(query_borrow) };
        let borrow = View::<Q> { query_borrow, view };
        self.view_borrow = Some(borrow);
    }
    pub fn unlock(&mut self) {
        self.view_borrow = None;
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
        return a.unwrap();
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
        let a = state.lock_mut(world);
        return a.unwrap();
    }

    fn apply(state: &mut Self::State, _: &crate::system::systemmeta::SystemMeta, _: &mut crate::World) {
        state.unlock();
    }
}
