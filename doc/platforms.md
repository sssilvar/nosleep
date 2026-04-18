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

## Linux (planned)

Linux has no single “sleep” API like macOS IOPM; behavior depends on **systemd**, **logind**, **desktop session** (X11 vs Wayland), and **idle monitors**.

Likely directions (to be chosen and implemented in `src/linux.rs` or similar):

| Mechanism | Role |
|-----------|------|
| `systemd-inhibit` / Inhibit [D-Bus API](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html) | Block idle/sleep/shutdown while the inhibitor FD or lease is open — common for CLI tools and headless use. |
| `org.freedesktop.ScreenSaver` / Portal “inhibit” | Session-level idle inhibition when a desktop and D-Bus are available. |
| **Tray** | `tray-icon` already supports GTK on Linux; same Quit UX as macOS once idle is wired. |

**Challenges**

- Wayland vs X11: where inhibition is exposed (portal vs legacy APIs) differs.


## Windows (not planned here yet)

Possible future work: `SetThreadExecutionState` / modern power APIs; tray already supported by `tray-icon` on Windows. Same module layout as `idle/macos.rs` / `idle/stub.rs`.
