# Platforms

`nosleep` asks the OS not to put the machine to **idle sleep** while the process runs.

Design: one small **idle** module per OS (see `src/idle/`), same CLI and tray UX everywhere we support.

## macOS (implemented)

**Idle sleep**

- [IOPMLib](https://developer.apple.com/documentation/iokit/iopmlib_h): `IOPMAssertionCreateWithName` with assertion type `PreventUserIdleSystemSleep` — same class of behavior as `caffeinate -i`.
- The assertion is held for the lifetime of the process and released on exit (Rust `Drop` on the guard).

**Tray**

- [tray-icon](https://crates.io/crates/tray-icon) + [tao](https://crates.io/crates/tao) event loop; menu item **Quit nosleep** ends the app.
- After creating the icon, the main run loop is woken with `CFRunLoopWakeUp` so the icon appears reliably (see `src/tray.rs`).

**Notes**

- Targets **system** idle sleep, not necessarily display power-off; display policy is separate from IOPM assertions.
- Sandboxed or restricted environments may limit power assertions — run like a normal user app from the terminal or a launcher.

## Linux (implemented)

**Idle sleep**

- [systemd-logind](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html) D-Bus `Inhibit` call with `what=idle`, `mode=block` — same scope as macOS's `PreventUserIdleDisplaySleep` (does not block explicit `systemctl suspend`).
- The inhibitor file descriptor is held open for the process lifetime (via `zbus` blocking API). Closing it — including on crash — automatically releases the lock.

**Tray**

- `tray-icon` + `tao` GTK backend; same Quit UX as macOS.

**Requirements**

- systemd/logind and D-Bus system bus must be available (standard on Debian, Ubuntu, Fedora, Arch, etc.).
- GTK 3 development headers (`libgtk-3-dev` / `gtk3-devel`).
- Ayatana AppIndicator (`libayatana-appindicator3-dev`) or legacy AppIndicator (`libappindicator3-dev`) for `tray-icon`.
- `libxdo-dev` — required by `tray-icon` on Linux (X11 event injection).

**Limitations**

- Non-systemd distros (Alpine musl, Void without runit-logind, etc.) are not supported; `prevent_idle()` will return `IdleError::System` at runtime.


## Windows (not planned here yet)

Possible future work: `SetThreadExecutionState` / modern power APIs; tray already supported by `tray-icon` on Windows. Same module layout as `idle/macos.rs` / `idle/stub.rs`.
