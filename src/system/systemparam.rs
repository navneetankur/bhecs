use crate::{ChangeTick, World};
use super::systemmeta::SystemMeta;
pub mod impls;

pub unsafe trait SystemParam: Sized {
    /// Used to store data which persists across invocations of a system.
    type State: Send + Sync + 'static;

    /// The item type returned when constructing this system param.
    /// The value of this associated type should be `Self`, instantiated with new lifetimes.
    ///
    /// You could think of `SystemParam::Item<'w, 's>` as being an *operation* that changes the lifetimes bound to `Self`.
    type Item<'world, 'state>: SystemParam<State = Self::State>;

    /// Registers any [`World`] access used by this [`SystemParam`]
    /// and creates a new instance of this param's [`State`](Self::State).
    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State;

    /// Applies any deferred mutations stored in this [`SystemParam`]'s state.
    /// This is used to apply [`Commands`] during [`apply_deferred`](crate::prelude::apply_deferred).
    ///
    /// [`Commands`]: crate::prelude::Commands
    #[inline]
    #[allow(unused_variables)]
    fn apply(state: &mut Self::State, system_meta: &SystemMeta, world: &mut World) {}

    /// Creates a parameter to be passed into a [`SystemParamFunction`].
    ///
    /// [`SystemParamFunction`]: super::SystemParamFunction
    ///
    /// # Safety
    ///
    /// - The passed [`UnsafeWorldCell`] must have access to any world data
    ///   registered in [`init_state`](SystemParam::init_state).
    /// - `world` must be the same `World` that was used to initialize [`state`](SystemParam::init_state).
    fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &SystemMeta,
        world: &'world World,
        change_tick: ChangeTick,
    ) -> Self::Item<'world, 'state>;
}
pub type SystemParamItem<'w, 's, P> = <P as SystemParam>::Item<'w, 's>;
macro_rules! impl_system_param_tuple {
    ($(#[$meta:meta])* $($param: ident),*) => {
        $(#[$meta])*
        // SAFETY: implementors of each `SystemParam` in the tuple have validated their impls
        #[allow(clippy::undocumented_unsafe_blocks)] // false positive by clippy
        #[allow(non_snake_case)]
        $(#[$meta])*
        unsafe impl<$($param: SystemParam),*> SystemParam for ($($param,)*) {
            type State = ($($param::State,)*);
            type Item<'w, 's> = ($($param::Item::<'w, 's>,)*);

            #[inline]
            fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
                (($($param::init_state(_world, _system_meta),)*))
            }
            #[inline]
            fn apply(($($param,)*): &mut Self::State, _system_meta: &SystemMeta, _world: &mut World) {
                $($param::apply($param, _system_meta, _world);)*
            }
            #[inline]
            #[allow(clippy::unused_unit)]
            fn get_param<'w, 's>(
                state: &'s mut Self::State,
                _system_meta: &SystemMeta,
                _world: &'w World,
                _change_tick: ChangeTick,
            ) -> Self::Item<'w, 's> {

                let ($($param,)*) = state;
                ($($param::get_param($param, _system_meta, _world, _change_tick),)*)
            }
        }
    };
}
// bevy_utils::all_tuples!( impl_system_param_tuple, 0, 16, P);
impl_system_param_tuple!();
impl_system_param_tuple!(P0);
impl_system_param_tuple!(P0, P1);
impl_system_param_tuple!(P0, P1, P2);
impl_system_param_tuple!(P0, P1, P2, P3);
impl_system_param_tuple!(P0, P1, P2, P3, P4);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
