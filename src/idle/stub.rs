// TODO: Windows — SetThreadExecutionState(ES_SYSTEM_REQUIRED | …) or Power API

use super::{IdleError, IdleGuard};

pub fn prevent_idle() -> Result<IdleGuard, IdleError> {
    Err(IdleError::UnsupportedPlatform)
}
