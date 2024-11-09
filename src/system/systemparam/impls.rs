use derive_more::derive::{Deref, DerefMut};
use hecs::{PreparedQueryBorrow};

use super::SystemParam;

#[derive(Deref, DerefMut, Default)]
pub struct PreparedQuery2<Q: hecs::Query>(pub hecs::PreparedQuery<Q>);
unsafe impl<Q: hecs::Query> Send for PreparedQuery2<Q>{}
unsafe impl<Q: hecs::Query> Sync for PreparedQuery2<Q>{}
impl<Q: hecs::Query> PreparedQuery2<Q> {
    #[must_use] pub fn new() -> Self { PreparedQuery2(hecs::PreparedQuery::new()) }
}

unsafe impl<Q: hecs::Query + 'static> SystemParam for PreparedQueryBorrow<'_, Q> 
{
    type State = PreparedQuery2<Q>;

    type Item<'world, 'state> = PreparedQueryBorrow<'world, Q>;

    fn init_state(_: &mut crate::World, _: &mut crate::system::systemmeta::SystemMeta) -> Self::State {
        PreparedQuery2(hecs::PreparedQuery::new())
    }

    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _: &crate::system::systemmeta::SystemMeta,
        world: &'world crate::World,
        _: crate::ChangeTick,
    ) -> Self::Item<'world, 'state> {
        let rv = state.query(world);
        // both 'world and 'state will live longer than this. So lifetimes are fine even
        // if rust compiler can't see it.
        // adding 'state: 'world. gives error https://github.com/rust-lang/rust/issues/100013
        // After it's resolved, let's try removing this transmute.
        let rv = unsafe { std::mem::transmute::<hecs::PreparedQueryBorrow<'_, Q>, hecs::PreparedQueryBorrow<'_, Q>>(rv) };
        return rv;
    }
}
