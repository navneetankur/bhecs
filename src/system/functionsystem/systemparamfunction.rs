use crate::system::{systeminput::SystemInput, systemparam::{SystemParam, SystemParamItem}};

pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    /// The input type of this system. See [`System::In`].
    type In: SystemInput;
    /// The return type of this system. See [`System::Out`].
    type Out;

    /// The [`SystemParam`]/s used by this system to access the [`World`].
    type Param: SystemParam;

    /// Executes this system once. See [`System::run`] or [`System::run_unsafe`].
    fn run(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        param_value: SystemParamItem<Self::Param>,
    ) -> Self::Out;
}
/// A marker type used to distinguish function systems with and without input.
#[doc(hidden)]
pub struct HasSystemInput;
macro_rules! impl_system_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func, $($param: SystemParam),*> SystemParamFunction<fn($($param,)*) -> Out> for Func
        where
            Func: Send + Sync + 'static,
            for <'a> &'a mut Func:
                FnMut($($param),*) -> Out +
                FnMut($(SystemParamItem<$param>),*) -> Out,
            Out: 'static
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, _input: (), param_value: SystemParamItem<($($param,)*)>) -> Out {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognize that `func`
                // is a function, potentially because of the multiple impls of `FnMut`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Out, $($param,)*>(
                    mut f: impl FnMut($($param,)*)->Out,
                    $($param: $param,)*
                )->Out{
                    f($($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, $($param),*)
            }
        }

        #[allow(non_snake_case)]
        impl<In, Out, Func, $($param: SystemParam),*> SystemParamFunction<(HasSystemInput, fn(In, $($param,)*) -> Out)> for Func
        where
            Func: Send + Sync + 'static,
            for <'a> &'a mut Func:
                FnMut(In, $($param),*) -> Out +
                FnMut(In::Param<'_>, $(SystemParamItem<$param>),*) -> Out,
            In: SystemInput + 'static,
            Out: 'static
        {
            type In = In;
            type Out = Out;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, input: In::Inner<'_>, param_value: SystemParamItem<($($param,)*)>) -> Out {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<In: SystemInput, Out, $($param,)*>(
                    mut f: impl FnMut(In::Param<'_>, $($param,)*)->Out,
                    input: In::Inner<'_>,
                    $($param: $param,)*
                )->Out{
                    f(In::wrap(input), $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, input, $($param),*)
            }
        }
    };
}
impl_system_function!();
impl_system_function!(P0);
impl_system_function!(P0, P1);
impl_system_function!(P0, P1, P2);
impl_system_function!(P0, P1, P2, P3);
impl_system_function!(P0, P1, P2, P3, P4);
impl_system_function!(P0, P1, P2, P3, P4, P5);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_system_function!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
// bevy_utils::all_tuples!(impl_system_function, 0, 16, F);
