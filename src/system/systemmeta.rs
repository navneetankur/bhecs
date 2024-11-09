use crate::ChangeTick;

#[derive(Default)]
pub struct SystemMeta {
    pub last_run: ChangeTick,
    pub has_deferred: bool,
}
