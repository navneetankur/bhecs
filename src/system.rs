pub mod systemparam;
pub mod exclusivesystemparam;
pub mod systemmeta;
pub mod systeminput;
pub mod functionsystem;
pub mod exclusivefunctionsystem;

use systeminput::{SystemIn, SystemInput};
use crate::World;
pub trait System: Send + Sync + 'static {
    /// The system's input.
    type In: SystemInput;
    /// The system's output.
    type Out;
    /// Returns true if the system must be run exclusively.
    fn is_exclusive(&self) -> bool;

    /// Returns true if system as deferred buffers
    fn has_deferred(&self) -> bool;

    fn run_unchecked(&mut self, input: SystemIn<'_, Self>, world: &mut World)
        -> Self::Out;

    fn run(&mut self, input: SystemIn<'_, Self>, world: &mut World)
        -> Self::Out {
        if !self.is_initialized(world) { self.initialize(world); }
        let rv = self.run_unchecked(input, world);
        self.apply_deferred(world);
        return rv;
    }


    /// Applies any [`Deferred`](crate::system::Deferred) system parameters (or other system buffers) of this system to the world.
    ///
    /// This is where [`Commands`](crate::system::Commands) get applied.
    fn apply_deferred(&mut self, world: &mut World);

    /// Initialize the system.
    fn initialize(&mut self, _world: &mut World);

    /// Initialize the system.
    fn is_initialized(&mut self, _world: &mut World) -> bool;
}
pub trait IntoSystem<In: SystemInput, Out, Marker>: Sized {
    /// The type of [`System`] that this instance converts into.
    type System: System<In = In, Out = Out>;

    /// Turns this value into its corresponding [`System`].
    fn into_system(this: Self) -> Self::System;
}

// All systems implicitly implement IntoSystem.
impl<T: System> IntoSystem<T::In, T::Out, ()> for T {
    type System = T;
    fn into_system(this: Self) -> Self {
        this
    }
}
