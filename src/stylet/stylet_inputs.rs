use crate::app::WindowState;
use crate::stylet::stylet_manager::{
    AxisEventState, ButtonEventState, ProximityEventState, StyletEvent, TipEventState,
};
use std::fs::OpenOptions;
use std::os::unix::{
    fs::OpenOptionsExt,
    io::{AsRawFd, OwnedFd},
};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui::{self, Vec2};
use input::event::tablet_tool::TabletToolType;
use input::{
    event::{
        tablet_tool::{TabletToolEvent, TabletToolEventTrait},
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
// pub fn spawn_pen_thread(stack: Arc<Mutex<Vec<TabletToolEvent>>>, window_state: Arc<Mutex<WindowState>>) {
pub fn spawn_pen_thread(
    window_state: Arc<Mutex<WindowState>>,
    events: Arc<Mutex<Vec<StyletEvent>>>,
    width: u32,
    height: u32,
) {
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
                if let Event::Tablet(tablet_event) = event {
                    // if !matches!(tablet_event, TabletToolEvent::Axis(_)){
                    //     println!("tablet_event: {tablet_event:?}");

                    // }
                    // println!("{:?}", tablet_event.tool().tool_type());
                    let pos = egui::pos2(
                        tablet_event.x_transformed(width) as f32 - window_pos.x * 2.,
                        tablet_event.y_transformed(height) as f32 - window_pos.y * 2., //todo ppp
                    );
                    let tooltype = tablet_event.tool().tool_type().unwrap_or_else(|| {
                        println!("No tool type, default: pen");
                        TabletToolType::Pen
                    });
                    match tablet_event {
                        TabletToolEvent::Axis(axis_event) => {
                            events
                                .lock()
                                .unwrap()
                                .push(StyletEvent::Axis(AxisEventState::new(
                                    pos,
                                    axis_event.pressure(),
                                    axis_event.distance(),
                                    axis_event.tilt_x(),
                                    axis_event.tilt_y(),
                                    tooltype,
                                )));
                        }
                        TabletToolEvent::Tip(tip_event) => {
                            events
                                .lock()
                                .unwrap()
                                .push(StyletEvent::Tip(TipEventState::new(
                                    pos,
                                    tip_event.pressure(),
                                    tip_event.distance(),
                                    tip_event.tilt_x(),
                                    tip_event.tilt_y(),
                                    tip_event.tip_state(),
                                    tooltype,
                                )));
                        }
                        TabletToolEvent::Proximity(proximity_event) => {
                            events.lock().unwrap().push(StyletEvent::Proximity(
                                ProximityEventState::new(
                                    pos,
                                    proximity_event.pressure(),
                                    proximity_event.distance(),
                                    proximity_event.tilt_x(),
                                    proximity_event.tilt_y(),
                                    proximity_event.proximity_state(),
                                    tooltype,
                                ),
                            ));
                        }
                        TabletToolEvent::Button(button_event) => {
                            events.lock().unwrap().push(StyletEvent::Button(
                                ButtonEventState::new(
                                    button_event.button(),
                                    button_event.button_state(),
                                    tooltype,
                                ),
                            ));
                        }
                        _ => todo!(),
                    }
                }

                //attention, bouton erase considéré à part
            }
        }
    });
}
