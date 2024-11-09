use core::marker::PhantomData;

use systemparamfunction::SystemParamFunction;

use crate::World;

use super::{systeminput::SystemIn, systemmeta::SystemMeta, systemparam::SystemParam, IntoSystem, System};

pub mod systemparamfunction;
pub struct FunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    pub(crate) func: F,
    pub(crate) param_state: Option<<F::Param as SystemParam>::State>,
    pub(crate) system_meta: SystemMeta,
    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    marker: PhantomData<fn() -> Marker>,
}
const PARAM_MESSAGE: &str = "System's param_state was not found. Did you forget to initialize this system before running it?";
#[doc(hidden)]
pub struct IsFunctionSystem;
impl<Marker, F> System for FunctionSystem<Marker, F>
where
    Marker: 'static,
    F: SystemParamFunction<Marker>,
{
    type In = F::In;
    type Out = F::Out;

    #[inline]
    fn is_exclusive(&self) -> bool {
        false
    }

    #[inline]
    fn has_deferred(&self) -> bool {
        self.system_meta.has_deferred
    }

    #[inline]
    fn run_unchecked(
        &mut self,
        input: SystemIn<'_, Self>,
        world: &mut World,
    ) -> Self::Out {
        let change_tick = world.increment_change_tick();

        // SAFETY:
        // - The caller has invoked `update_archetype_component_access`, which will panic
        //   if the world does not match.
        // - All world accesses used by `F::Param` have been registered, so the caller
        //   will ensure that there are no data access conflicts.
        let params = {
            F::Param::get_param(
                self.param_state.as_mut().expect(PARAM_MESSAGE),
                &self.system_meta,
                world,
                change_tick,
            )
        };
        let out = self.func.run(input, params);
        self.system_meta.last_run = change_tick;
        out
    }

    #[inline]
    fn apply_deferred(&mut self, world: &mut World) {
        let param_state = self.param_state.as_mut().expect(PARAM_MESSAGE);
        F::Param::apply(param_state, &self.system_meta, world);
    }

    #[inline]
    fn initialize(&mut self, world: &mut World) {
        self.param_state = Some(F::Param::init_state(world, &mut self.system_meta));
        self.system_meta.last_run = 0;
    }

    fn is_initialized(&mut self, _world: &mut World) -> bool {
        self.param_state.is_some()
    }
}


impl<Marker, F> IntoSystem<F::In, F::Out, (IsFunctionSystem, Marker)> for F
where
    Marker: 'static,
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<Marker, F>;
    fn into_system(func: Self) -> Self::System {
        FunctionSystem {
            func,
            param_state: None,
            system_meta: SystemMeta::default(),
            marker: PhantomData,
        }
    }
}
