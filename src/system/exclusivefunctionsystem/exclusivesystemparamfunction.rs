use crate::{system::{exclusivesystemparam::{ExclusiveSystemParam, ExclusiveSystemParamItem}, systeminput::SystemInput}, World};

pub trait ExclusiveSystemParamFunction<Marker>: Send + Sync + 'static {
    /// The input type to this system. See [`System::In`].
    type In: SystemInput;

    /// The return type of this system. See [`System::Out`].
    type Out;

    /// The [`ExclusiveSystemParam`]'s defined by this system's `fn` parameters.
    type Param: ExclusiveSystemParam;

    /// Executes this system once. See [`System::run`].
    fn run(
        &mut self,
        world: &mut World,
        input: <Self::In as SystemInput>::Inner<'_>,
        param_value: ExclusiveSystemParamItem<Self::Param>,
    ) -> Self::Out;
}
#[doc(hidden)]
pub struct HasExclusiveSystemInput;
macro_rules! impl_exclusive_system_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func, $($param: ExclusiveSystemParam),*> ExclusiveSystemParamFunction<fn($($param,)*) -> Out> for Func
        where
            Func: Send + Sync + 'static,
            for <'a> &'a mut Func:
                FnMut(&mut World, $($param),*) -> Out +
                FnMut(&mut World, $(ExclusiveSystemParamItem<$param>),*) -> Out,
            Out: 'static,
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, world: &mut World, _in: (), param_value: ExclusiveSystemParamItem< ($($param,)*)>) -> Out {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognize that `func`
                // is a function, potentially because of the multiple impls of `FnMut`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Out, $($param,)*>(
                    mut f: impl FnMut(&mut World, $($param,)*) -> Out,
                    world: &mut World,
                    $($param: $param,)*
                ) -> Out {
                    f(world, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, world, $($param),*)
            }
        }

        #[allow(non_snake_case)]
        impl<In, Out, Func, $($param: ExclusiveSystemParam),*> ExclusiveSystemParamFunction<(HasExclusiveSystemInput, fn(In, $($param,)*) -> Out)> for Func
        where
            Func: Send + Sync + 'static,
            for <'a> &'a mut Func:
                FnMut(In, &mut World, $($param),*) -> Out +
                FnMut(In::Param<'_>, &mut World, $(ExclusiveSystemParamItem<$param>),*) -> Out,
            In: SystemInput + 'static,
            Out: 'static,
        {
            type In = In;
            type Out = Out;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, world: &mut World, input: In::Inner<'_>, param_value: ExclusiveSystemParamItem< ($($param,)*)>) -> Out {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognize that `func`
                // is a function, potentially because of the multiple impls of `FnMut`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<In: SystemInput, Out, $($param,)*>(
                    mut f: impl FnMut(In::Param<'_>, &mut World, $($param,)*) -> Out,
                    input: In::Inner<'_>,
                    world: &mut World,
                    $($param: $param,)*
                ) -> Out {
                    f(In::wrap(input), world, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, input, world, $($param),*)
            }
        }
    };
}
impl_exclusive_system_function!();
impl_exclusive_system_function!(P0);
impl_exclusive_system_function!(P0, P1);
impl_exclusive_system_function!(P0, P1, P2);
impl_exclusive_system_function!(P0, P1, P2, P3);
impl_exclusive_system_function!(P0, P1, P2, P3, P4);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_exclusive_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
