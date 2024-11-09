use super::System;

pub trait SystemInput: Sized {
    /// The wrapper input type that is defined as the first argument to [`FunctionSystem`]s.
    ///
    /// [`FunctionSystem`]: crate::system::FunctionSystem
    type Param<'i>: SystemInput;
    /// The inner input type that is passed to functions that run systems,
    /// such as [`System::run`].
    ///
    /// [`System::run`]: crate::system::System::run
    type Inner<'i>;

    /// Converts a [`SystemInput::Inner`] into a [`SystemInput::Param`].
    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_>;
}
pub type SystemIn<'a, S> = <<S as System>::In as SystemInput>::Inner<'a>;
impl SystemInput for () {
    type Param<'i> = ();
    type Inner<'i> = ();

    fn wrap(_this: Self::Inner<'_>) -> Self::Param<'_> {}
}
