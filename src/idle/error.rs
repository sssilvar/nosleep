use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdleError {
    #[cfg_attr(any(target_os = "macos", target_os = "linux"), allow(dead_code))]
    #[error("idle assertion: not implemented on this OS yet")]
    UnsupportedPlatform,

    #[error("idle assertion: {0}")]
    System(&'static str),

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    #[error("IOPM assertion failed (IOReturn {0})")]
    AssertionFailed(i32),
}
