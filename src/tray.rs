//! Menu bar status icon (Docker-style) + Quit; macOS wakes main run loop after create.

use std::time::{Duration, Instant};

use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};

enum UserEvent {
    Menu(MenuEvent),
    Timeout,
}

/// Blocks until Quit or optional deadline. Does not return (`tao` event loop).
pub fn run_until(until: Option<Instant>) -> ! {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy_menu = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy_menu.send_event(UserEvent::Menu(event));
    }));

    if let Some(deadline) = until {
        let proxy = event_loop.create_proxy();
        std::thread::spawn(move || {
            while Instant::now() < deadline {
                std::thread::sleep(Duration::from_secs(1));
            }
            let _ = proxy.send_event(UserEvent::Timeout);
        });
    }

    let icon = icon();

    let menu = Menu::new();
    let quit = MenuItem::new("Quit nosleep", true, None);
    if menu.append(&quit).is_err() {
        std::process::exit(1);
    }

    let mut tray_icon = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                match TrayIconBuilder::new()
                    .with_menu(Box::new(menu.clone()))
                    .with_tooltip("nosleep — preventing idle sleep and display sleep")
                    .with_icon(icon.clone())
                    .build()
                {
                    Ok(t) => {
                        tray_icon = Some(t);
                        #[cfg(target_os = "macos")]
                        {
                            use objc2_core_foundation::CFRunLoop;
                            if let Some(rl) = CFRunLoop::main() {
                                rl.wake_up();
                            }
                        }
                    }
                    Err(_) => {
                        *control_flow = ControlFlow::ExitWithCode(1);
                    }
                }
            }

            Event::UserEvent(UserEvent::Menu(ev)) if ev.id == quit.id() => {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }

            Event::UserEvent(UserEvent::Menu(_)) => {}

            Event::UserEvent(UserEvent::Timeout) => {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }

            _ => {}
        }
    })
}

/// Small filled circle (green), template-friendly on macOS.
fn icon() -> Icon {
    const S: u32 = 22;
    let mut rgba = vec![0u8; (S * S * 4) as usize];
    let cx = S as f32 / 2.0;
    let cy = S as f32 / 2.0;
    let r = S as f32 / 2.0 - 2.0;
    for y in 0..S {
        for x in 0..S {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let i = ((y * S + x) * 4) as usize;
            if dx * dx + dy * dy <= r * r {
                rgba[i] = 0x34;
                rgba[i + 1] = 0xc7;
                rgba[i + 2] = 0x59;
                rgba[i + 3] = 0xff;
            }
        }
    }
    Icon::from_rgba(rgba, S, S).expect("tray icon rgba")
}
