# nosleep

Keep your machine from going to **idle sleep** while running long jobs.

| Platform | Idle prevention | Tray “running” indicator |
|----------|-----------------|---------------------------|
| **macOS** | Yes ([IOPM](https://developer.apple.com/documentation/iokit/iopmlib_h) assertion, like `caffeinate -i`) | Yes (menu bar) |
| **Linux** | Planned | Planned (same UX) |
| **Windows** | Not yet | — |

How each OS is (or will be) implemented: **[doc/platforms.md](doc/platforms.md)**.

## Requirements

- **Rust** (recent stable; this crate uses edition 2024).
- **macOS** today for a working build of idle prevention; **Linux support is intended**—see [doc/platforms.md](doc/platforms.md#linux-planned).

### Linux build dependencies

The tray icon links against GTK 3, D-Bus, libxdo and AppIndicator, so their development
packages must be present before `cargo install`.

Debian / Ubuntu:

```sh
sudo apt install pkg-config libgtk-3-dev libxdo-dev libdbus-1-dev libayatana-appindicator3-dev
```

Fedora:

```sh
sudo dnf install pkgconf-pkg-config gtk3-devel libxdo-devel dbus-devel libappindicator-gtk3-devel
```

Arch:

```sh
sudo pacman -S --needed pkgconf gtk3 xdotool dbus libayatana-appindicator
```

If `cargo install` fails with `The system library \`gdk-3.0\` required by crate \`gdk-sys\` was not
found`, one of these packages is missing.

## Install (Cargo)

```sh
cargo install --git https://github.com/sssilvar/nosleep
```

Installs to Cargo’s bin dir (often `~/.cargo/bin`). You'' need to add it to your `PATH`.

## Usage

```text
nosleep [MINUTES]
```

- **No argument** — runs until **Quit nosleep** in the tray/menu bar menu (macOS).
- **`MINUTES`** — exit automatically after that many minutes (e.g. `nosleep 120`).

Starting from a terminal: the tray icon stays up while the process runs; closing the terminal may kill the app unless you detach it (`nohup`, background job, etc.).

## License

[MIT](LICENSE).
