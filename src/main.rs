//! `nosleep` — `PreventUserIdleSystemSleep` IOPM assertion (macOS). Menu bar icon while active (like Docker).
mod idle;
mod tray;

use std::time::{Duration, Instant};

use clap::Parser;

#[derive(Parser)]
#[command(name = "nosleep")]
struct Args {
    /// Stop after N minutes (omit = run until Quit from menu bar)
    minutes: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let until = args
        .minutes
        .map(|m| Instant::now() + Duration::from_secs(m.saturating_mul(60)));

    let _idle = idle::prevent_user_idle_system_sleep()?;
    tray::run_until(until);
}
