use derive_more::derive::{Deref, DerefMut};
use hecs::{PreparedQuery, PreparedQueryBorrow, PreparedQueryIter, PreparedView, World};

use crate::system::systemparam::SystemParam;

pub struct ViewState<Q: hecs::Query + 'static> {
    query: PreparedQuery<Q>,
    borrow: Option<PreparedQueryBorrow<'static, Q>>,
}
unsafe impl<Q: hecs::Query> Send for ViewState<Q>{}
unsafe impl<Q: hecs::Query> Sync for ViewState<Q>{}
impl<Q: hecs::Query> Default for ViewState<Q> {
    fn default() -> Self {
        Self { query: Default::default(), borrow: Default::default() }
    }
}
impl<Q: hecs::Query> ViewState<Q> {
    pub fn lock<'w>(&mut self, world: &'w World) -> Option<PreparedView<'w, Q>> {
        use core::mem::transmute;
        let mut query_borrow = self.query.query(world);
        let view = query_borrow.view();
        // # Safety
        // transmut changes lifetimes. So ensure lifetime gurantees manually.
        // resource is borrowed for world. So no dangling pointer.
        // lock by hecs on this column, will be gone when query_borrow drops.
        // self.unlock() will drop it. When called from system.apply.
        let view = unsafe { transmute::<hecs::PreparedView<'_, Q>, hecs::PreparedView<'_, Q>>(view) };
        let borrow = unsafe { transmute::<hecs::PreparedQueryBorrow<'_, Q>, hecs::PreparedQueryBorrow<'_, Q>>(query_borrow) };
        self.borrow = Some(borrow);
        // # Safety. Borrow is actually from world too. It won't live past function call.
        // unlock called by system.apply will remove hecs lock.
        return Some(view);
    }
    pub fn unlock(&mut self) {
        self.borrow = None;
    }
}

impl<Q: hecs::Query + 'static> SystemParam for PreparedView<'_, Q> 
{
    type State = ViewState<Q>;

    type Item<'world, 'state> = PreparedView<'world, Q>;

    fn init_state(_: &mut crate::World, _: &mut crate::system::systemmeta::SystemMeta) -> Self::State {
        ViewState::<Q>::default()
    }

    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _: &crate::system::systemmeta::SystemMeta,
        world: &'world crate::World,
        _: crate::ChangeTick,
    ) -> Self::Item<'world, 'state> {
        state.lock(world).unwrap()
    }

    fn apply(state: &mut Self::State, _: &crate::system::systemmeta::SystemMeta, _: &mut crate::World) {
        state.unlock();
    }
}
