
use crate::app::{PenState, WindowState};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd}};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui;
use input::{
    event::{
        tablet_tool::{TabletToolEvent, TabletToolEventTrait, TipState},
        Event,
    },
    Libinput, LibinputInterface,
};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(true)
            .write(true)
            .open(path)
            //.map(|f| unsafe { OwnedFd::from_raw_fd(f.into_raw_fd()) })
            .map(OwnedFd::from)
            .map_err(|e| e.raw_os_error().unwrap_or(-1))
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}
// --- Interface libinput ---
#[allow(unsafe_code)]
pub fn spawn_pen_thread(state: Arc<Mutex<PenState>>, window_state: Arc<Mutex<WindowState>>) {
    thread::spawn(move || {
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();

        let fd = input.as_raw_fd();

        loop {
            let mut pollfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            unsafe { libc::poll(&mut pollfd, 1, -1) };

            input.dispatch().unwrap();
            let window_pos = window_state.lock().unwrap().pos;

            for event in &mut input {
                if let Event::Tablet(TabletToolEvent::Axis(e)) = &event {
                    let mut s = state.lock().unwrap();
                    
                    s.pos = egui::pos2(
                        e.x_transformed(1920) as f32 - window_pos.x
                        ,
                        e.y_transformed(1080) as f32 - window_pos.y,
                    );
                }
                if let Event::Tablet(TabletToolEvent::Tip(e)) = &event {
                    let mut s = state.lock().unwrap();
                    s.pressed = matches!(e.tip_state(), TipState::Down);
                }
            }
        }
    });
}
