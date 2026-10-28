use std::sync::Arc;

use parking_lot::Mutex;
use winput::Action;
use winput::message_loop;
use winput::message_loop::Event;

use crate::core::FocusFollowsMouseImplementation;

use crate::window_manager::WindowManager;

#[tracing::instrument]
pub fn listen_for_movements(wm: Arc<Mutex<WindowManager>>) {
    std::thread::spawn(move || {
        let mut ignore_movement = false;

        let receiver = message_loop::start().expect("could not start winput message loop");

        loop {
            // Always block on the next event. Checking the implementation
            // before waiting turns this into a busy loop while ffm is disabled.
            match receiver.next_event() {
                // Don't want to send any raise events while we are dragging or resizing
                Event::MouseButton { action, .. } => match action {
                    Action::Press => ignore_movement = true,
                    Action::Release => ignore_movement = false,
                },
                Event::MouseMoveRelative { .. } if !ignore_movement => {
                    let mut wm = wm.lock();
                    if matches!(
                        wm.focus_follows_mouse,
                        Some(FocusFollowsMouseImplementation::Komorebi)
                    ) && let Err(error) = wm.raise_window_at_cursor_pos()
                    {
                        tracing::error!("{}", error);
                    }
                }
                _ => {}
            }
        }
    });
}
