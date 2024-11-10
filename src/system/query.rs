use derive_more::derive::{Deref, DerefMut};
use hecs::{PreparedQuery, PreparedQueryBorrow, PreparedQueryIter, World};

use super::systemparam::SystemParam;
pub struct QueryIterState<Q: hecs::Query + 'static> {
    query: PreparedQuery<Q>,
    borrow: Option<PreparedQueryBorrow<'static, Q>>,
}
unsafe impl<Q: hecs::Query> Send for QueryIterState<Q>{}
unsafe impl<Q: hecs::Query> Sync for QueryIterState<Q>{}
impl<Q: hecs::Query> Default for QueryIterState<Q> {
    fn default() -> Self {
        Self { query: Default::default(), borrow: Default::default() }
    }
}
impl<Q: hecs::Query> QueryIterState<Q> {
    pub fn lock<'w>(&mut self, world: &'w World) -> Option<PreparedQueryIter<'w, Q>> {
        use core::mem::transmute;
        let mut query_borrow = self.query.query(world);
        let iter = query_borrow.iter();
        // # Safety
        // transmut changes lifetimes. So ensure lifetime gurantees manually.
        // resource is borrowed for world. So no dangling pointer.
        // lock by hecs on this column, will be gone when query_borrow drops.
        // self.unlock() will drop it. When called from system.apply.
        let iter = unsafe { transmute::<hecs::PreparedQueryIter<'_, Q>, hecs::PreparedQueryIter<'_, Q>>(iter) };
        let borrow = unsafe { transmute::<hecs::PreparedQueryBorrow<'_, Q>, hecs::PreparedQueryBorrow<'_, Q>>(query_borrow) };
        self.borrow = Some(borrow);
        // # Safety. Borrow is actually from world too. It won't live past function call.
        // unlock called by system.apply will remove hecs lock.
        return Some(iter);
    }
    pub fn unlock(&mut self) {
        self.borrow = None;
    }
}

impl<Q: hecs::Query + 'static> SystemParam for PreparedQueryIter<'_, Q> 
{
    type State = QueryIterState<Q>;

    type Item<'world, 'state> = PreparedQueryIter<'world, Q>;

    fn init_state(_: &mut crate::World, _: &mut crate::system::systemmeta::SystemMeta) -> Self::State {
        QueryIterState::default()
    }

    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _: &crate::system::systemmeta::SystemMeta,
        world: &'world crate::World,
        _: crate::ChangeTick,
    ) -> Self::Item<'world, 'state> {
        state.lock(world).unwrap()
    }

    fn apply(state: &mut Self::State, _: &super::systemmeta::SystemMeta, _: &mut crate::World) {
        state.unlock();
    }
}
