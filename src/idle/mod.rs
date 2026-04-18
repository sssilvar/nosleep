//! Idle-sleep prevention via OS power APIs (macOS IOPM; others TODO).

mod error;

pub use error::IdleError;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::prevent_user_idle_system_sleep;

#[cfg(not(target_os = "macos"))]
mod stub;
#[cfg(not(target_os = "macos"))]
pub use stub::prevent_user_idle_system_sleep;

/// Active IOPM assertion on macOS; releasing drops idle-sleep prevention.
pub struct IdleGuard {
    #[cfg(target_os = "macos")]
    pub(super) id: u32,
    #[cfg(not(target_os = "macos"))]
    pub(super) _private: (),
}
