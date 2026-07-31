//! logind `Inhibit` — holds an `idle` inhibitor fd for the process lifetime,
//! plus an `org.freedesktop.ScreenSaver` inhibit (logind's idle lock does not
//! stop GNOME/KDE from blanking and locking the screen).

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

    Ok(IdleGuard {
        _fd: fd,
        _screensaver: inhibit_screensaver(),
    })
}

/// Session-bus screensaver inhibit; the lock lives as long as this connection.
/// Absent on bare WMs, so failure is not fatal.
fn inhibit_screensaver() -> Option<Connection> {
    let conn = Connection::session().ok()?;
    conn.call_method(
        Some("org.freedesktop.ScreenSaver"),
        "/org/freedesktop/ScreenSaver",
        Some("org.freedesktop.ScreenSaver"),
        "Inhibit",
        &("nosleep", "nosleep is active"),
    )
    .ok()?;
    Some(conn)
}
