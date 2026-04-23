//! Idle-sleep prevention via OS power APIs (macOS IOPM, Linux logind).

mod error;

pub use error::IdleError;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::prevent_idle;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::prevent_idle;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
mod stub;
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub use stub::prevent_idle;

/// Active sleep-prevention guard; releasing drops idle-sleep prevention.
pub struct IdleGuard {
    #[cfg(target_os = "macos")]
    pub(super) id: u32,
    /// Logind inhibitor fd — kept alive until drop.
    #[cfg(target_os = "linux")]
    pub(super) _fd: zbus::zvariant::OwnedFd,
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub(super) _private: (),
}
