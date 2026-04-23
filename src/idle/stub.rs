// TODO: Windows — SetThreadExecutionState(ES_SYSTEM_REQUIRED | …) or Power API
// TODO: Linux — org.freedesktop.ScreenSaver Inhibit, systemd-inhibit, or /dev/input grab — pick one policy

use super::{IdleError, IdleGuard};

pub fn prevent_idle() -> Result<IdleGuard, IdleError> {
    Err(IdleError::UnsupportedPlatform)
}
