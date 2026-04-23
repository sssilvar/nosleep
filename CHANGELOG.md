# [0.2.0] - 2026-04-23

### Fixed

- Screen no longer turns off while `nosleep` is active — switched IOPM assertion from `PreventUserIdleSystemSleep` to `PreventUserIdleDisplaySleep`, which covers both display and system idle sleep.

### Changed

- Renamed internal `prevent_user_idle_system_sleep` to `prevent_idle`.

# [0.1.0] - 2026-04-18

### Added

- macOS idle sleep prevention via `IOPMAssertionCreateWithName` (`PreventUserIdleSystemSleep`) — same class of behavior as `caffeinate -i`.
- Menu bar tray icon with **Quit nosleep** menu item.
- Optional `MINUTES` argument to exit automatically after a set duration.
- Platform stub for non-macOS targets returning `IdleError::UnsupportedPlatform`.
- README and platform implementation notes (`doc/platforms.md`).
