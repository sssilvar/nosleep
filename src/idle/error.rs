use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdleError {
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    #[error("idle assertion: not implemented on this OS yet")]
    UnsupportedPlatform,

    #[error("idle assertion: {0}")]
    System(&'static str),

    #[error("IOPM assertion failed (IOReturn {0})")]
    AssertionFailed(i32),
}
