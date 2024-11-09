use crate::World;

use super::systemmeta::SystemMeta;

pub trait ExclusiveSystemParam: Sized {
    /// Used to store data which persists across invocations of a system.
    type State: Send + Sync + 'static;
    /// The item type returned when constructing this system param.
    /// See [`SystemParam::Item`].
    type Item<'s>: ExclusiveSystemParam<State = Self::State>;

    /// Creates a new instance of this param's [`State`](Self::State).
    fn init(world: &mut World, system_meta: &mut SystemMeta) -> Self::State;

    /// Creates a parameter to be passed into an [`ExclusiveSystemParamFunction`].
    ///
    /// [`ExclusiveSystemParamFunction`]: super::ExclusiveSystemParamFunction
    fn get_param<'s>(state: &'s mut Self::State, system_meta: &SystemMeta) -> Self::Item<'s>;
}
pub type ExclusiveSystemParamItem<'s, P> = <P as ExclusiveSystemParam>::Item<'s>;
macro_rules! impl_exclusive_system_param_tuple {
    ($(#[$meta:meta])* $($param: ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        $(#[$meta])*
        impl<$($param: ExclusiveSystemParam),*> ExclusiveSystemParam for ($($param,)*) {
            type State = ($($param::State,)*);
            type Item<'s> = ($($param::Item<'s>,)*);

            #[inline]
            fn init(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
                (($($param::init(_world, _system_meta),)*))
            }

            #[inline]
            #[allow(clippy::unused_unit)]
            fn get_param<'s>(
                state: &'s mut Self::State,
                system_meta: &SystemMeta,
            ) -> Self::Item<'s> {

                let ($($param,)*) = state;
                ($($param::get_param($param, system_meta),)*)
            }
        }
    };
}
impl_exclusive_system_param_tuple!();
impl_exclusive_system_param_tuple!(P0);
impl_exclusive_system_param_tuple!(P0, P1);
impl_exclusive_system_param_tuple!(P0, P1, P2);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_exclusive_system_param_tuple!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
