use hecs::Component;

use crate::system::systeminput::SystemInput;

pub trait Event: Component {}
// impl<E: Component> Event for E {}

// impl<E: Event> SystemInput for E {
//     type Param<'i> = E;

//     type Inner<'i> = E;

//     fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
//         this
//     }
// }

impl<E: Event> SystemInput for &E {
    type Param<'i> = &'i E;

    type Inner<'i> = &'i E;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
        this
    }
}
