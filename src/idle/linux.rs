//! logind `Inhibit` — holds an `idle` inhibitor fd for the process lifetime.

use zbus::blocking::Connection;
use zbus::zvariant::OwnedFd;

use super::{IdleError, IdleGuard};

pub fn prevent_idle() -> Result<IdleGuard, IdleError> {
    let conn =
        Connection::system().map_err(|_| IdleError::System("D-Bus system bus unavailable"))?;

    let reply = conn
        .call_method(
            Some("org.freedesktop.login1"),
            "/org/freedesktop/login1",
            Some("org.freedesktop.login1.Manager"),
            "Inhibit",
            &("idle", "nosleep", "nosleep is active", "block"),
        )
        .map_err(|_| IdleError::System("logind Inhibit call failed"))?;

    let fd: OwnedFd = reply
        .body()
        .deserialize()
        .map_err(|_| IdleError::System("unexpected reply from logind"))?;

    Ok(IdleGuard { _fd: fd })
}
