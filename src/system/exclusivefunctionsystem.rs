use core::marker::PhantomData;

use exclusivesystemparamfunction::ExclusiveSystemParamFunction;

use crate::World;

use super::{exclusivesystemparam::ExclusiveSystemParam, systeminput::SystemIn, systemmeta::SystemMeta, IntoSystem, System};

pub mod exclusivesystemparamfunction;
pub struct ExclusiveFunctionSystem<Marker, F>
where
    F: ExclusiveSystemParamFunction<Marker>,
{
    func: F,
    param_state: Option<<F::Param as ExclusiveSystemParam>::State>,
    system_meta: SystemMeta,
    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    marker: PhantomData<fn() -> Marker>,
}
#[doc(hidden)]
pub struct IsExclusiveFunctionSystem;
const PARAM_MESSAGE: &str = "System's param_state was not found. Did you forget to initialize this system before running it?";
impl<Marker, F> System for ExclusiveFunctionSystem<Marker, F>
where
    Marker: 'static,
    F: ExclusiveSystemParamFunction<Marker>,
{
    type In = F::In;
    type Out = F::Out;

    #[inline]
    fn is_exclusive(&self) -> bool {
        true
    }

    #[inline]
    fn has_deferred(&self) -> bool {
        // exclusive systems have no deferred system params
        false
    }

    fn run_unchecked(&mut self, input: SystemIn<'_, Self>, world: &mut World) -> Self::Out {
        let params = F::Param::get_param(
            self.param_state.as_mut().expect(PARAM_MESSAGE),
            &self.system_meta,
        );
        let out = self.func.run(world, input, params);
        self.system_meta.last_run = world.increment_change_tick();
        return out;
    }

    #[inline]
    fn apply_deferred(&mut self, _world: &mut World) {
        // "pure" exclusive systems do not have any buffers to apply.
        // Systems made by piping a normal system with an exclusive system
        // might have buffers to apply, but this is handled by `PipeSystem`.
    }

    #[inline]
    fn initialize(&mut self, world: &mut World) {
        self.system_meta.last_run = 0;
        self.param_state = Some(F::Param::init(world, &mut self.system_meta));
    }

    fn is_initialized(&mut self, _: &mut World) -> bool {
        self.param_state.is_some()
    }
}
impl<Marker, F> IntoSystem<F::In, F::Out, (IsExclusiveFunctionSystem, Marker)> for F
where
    Marker: 'static,
    F: ExclusiveSystemParamFunction<Marker>,
{
    type System = ExclusiveFunctionSystem<Marker, F>;
    fn into_system(func: Self) -> Self::System {
        ExclusiveFunctionSystem {
            func,
            param_state: None,
            system_meta: SystemMeta::default(),
            marker: PhantomData,
        }
    }
}
