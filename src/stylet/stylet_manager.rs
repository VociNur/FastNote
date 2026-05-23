use std::sync::{Arc, Mutex};

use eframe::egui::{self, Pos2, Rect, Vec2};
use input::event::{
    pointer::ButtonState,
    tablet_tool::{ProximityState, TabletToolType, TipState},
};

use crate::stylet::stylet::StyletState;

#[derive(Default)]
pub struct StyletManager {
    pub stylet: StyletState,
    pub events: Arc<Mutex<Vec<StyletEvent>>>, //un peu galère pas de trait copy
}

impl StyletManager {
    pub fn manage_events(self: &mut Self, has_focus: &bool, gpu_rect: &Option<Rect>, clicks: &mut Vec<Vec2>) {
        let events = std::mem::take(&mut *self.events.lock().unwrap());
        for event in events {
            match event {
                StyletEvent::Axis(axis_event_state) => {
                    self.on_axis_event(&axis_event_state, gpu_rect, clicks);
                    self.stylet.pos = axis_event_state.pos;
                    self.stylet.pressure = axis_event_state.pressure;
                    self.stylet.distance = axis_event_state.distance;
                    self.stylet.tilt_x = axis_event_state.tilt_x;
                    self.stylet.tilt_y = axis_event_state.tilt_y;
                    self.stylet.tool_type = axis_event_state.tool_type;
                }
                StyletEvent::Tip(tip_event_state) => {
                    self.on_tip_event(&tip_event_state, gpu_rect, clicks);
                    self.stylet.pos = tip_event_state.pos;
                    self.stylet.pressure = tip_event_state.pressure;
                    self.stylet.distance = tip_event_state.distance;
                    self.stylet.tilt_x = tip_event_state.tilt_x;
                    self.stylet.tilt_y = tip_event_state.tilt_y;
                    self.stylet.pressed = tip_event_state.tip_state == TipState::Down;
                    self.stylet.tool_type = tip_event_state.tool_type;
                }
                StyletEvent::Proximity(proximity_event_state) => {
                    self.on_proximity_event(&proximity_event_state);
                    self.stylet.pos = proximity_event_state.pos;
                    self.stylet.pressure = proximity_event_state.pressure;
                    self.stylet.distance = proximity_event_state.distance;
                    self.stylet.tilt_x = proximity_event_state.tilt_x;
                    self.stylet.tilt_y = proximity_event_state.tilt_y;
                    self.stylet.in_proximity =
                        proximity_event_state.proximity_state == ProximityState::In;
                    self.stylet.tool_type = proximity_event_state.tool_type;
                }
                StyletEvent::Button(button_event_state) => {
                    self.on_button_event(&button_event_state);
                    self.stylet.tool_type = button_event_state.tool_type;
                }
            }
        }
    }
    pub fn touch_gpu(self: &mut Self, pos: Pos2, opt_gpu_rect: &Option<Rect>, clicks: &mut Vec<Vec2>){
        if !self.stylet.pressed{
            return
        }
        if opt_gpu_rect.is_none(){
            println!("Gpu rect is none\n Return\n");
            return;
        }
        let gpu_rect = opt_gpu_rect.unwrap();
            if gpu_rect.contains(pos){
                println!("Inside the gpu rect");
                let draw_pos = pos - gpu_rect.min;
                clicks.push(draw_pos);
                
            }else{
                println!("Outside the gpu rect");
                
            }
    } 
    pub fn on_axis_event(self: &mut Self, axis_event_state: &AxisEventState, opt_gpu_rect: &Option<Rect>, clicks: &mut Vec<Vec2>) {
        // println!("axis: {axis_event_state:?}");
        println!("stylet: {}", axis_event_state.pos);
        self.touch_gpu(axis_event_state.pos, opt_gpu_rect, clicks);
    }

    pub fn on_tip_event(self: &mut Self, tip_event_state: &TipEventState, opt_gpu_rect: &Option<Rect>, clicks: &mut Vec<Vec2>) {
        // println!("tip: {tip_event_state:?}");
        self.touch_gpu(tip_event_state.pos, opt_gpu_rect, clicks);
    }

    pub fn on_proximity_event(self: &mut Self, proximity_event_state: &ProximityEventState) {
        // println!("proximity: {proximity_event_state:?}");
    }

    pub fn on_button_event(self: &mut Self, button_event_state: &ButtonEventState) {
        println!("button: {button_event_state:?}");
    }
}

//ENUM
pub enum StyletEvent {
    Axis(AxisEventState),
    Tip(TipEventState),
    Proximity(ProximityEventState),
    Button(ButtonEventState),
}

#[derive(Debug)]
pub struct AxisEventState {
    pos: egui::Pos2,
    pressure: f64,
    distance: f64,
    tilt_x: f64,
    tilt_y: f64,
    tool_type: TabletToolType,
    // slider osef pour moi
}

impl AxisEventState {
    pub fn new(
        pos: egui::Pos2,
        pressure: f64,
        distance: f64,
        tilt_x: f64,
        tilt_y: f64,
        tool_type: TabletToolType,
    ) -> Self {
        Self {
            pos,
            pressure,
            distance,
            tilt_x,
            tilt_y,
            tool_type,
        }
    }
}
#[derive(Debug)]
pub struct TipEventState {
    pos: egui::Pos2,
    pressure: f64,
    distance: f64,
    tilt_x: f64,
    tilt_y: f64,
    tip_state: TipState,
    tool_type: TabletToolType,
}

impl TipEventState {
    pub fn new(
        pos: egui::Pos2,
        pressure: f64,
        distance: f64,
        tilt_x: f64,
        tilt_y: f64,
        tip_state: TipState,
        tool_type: TabletToolType,
    ) -> Self {
        Self {
            pos,
            pressure,
            distance,
            tilt_x,
            tilt_y,
            tip_state,
            tool_type,
        }
    }
}

#[derive(Debug)]
pub struct ProximityEventState {
    pos: egui::Pos2,
    pressure: f64,
    distance: f64,
    tilt_x: f64,
    tilt_y: f64,
    proximity_state: ProximityState,
    tool_type: TabletToolType,
}
impl ProximityEventState {
    pub fn new(
        pos: egui::Pos2,
        pressure: f64,
        distance: f64,
        tilt_x: f64,
        tilt_y: f64,
        proximity_state: ProximityState,
        tool_type: TabletToolType,
    ) -> Self {
        Self {
            pos,
            pressure,
            distance,
            tilt_x,
            tilt_y,
            proximity_state,
            tool_type,
        }
    }
}

#[derive(Debug)]
pub struct ButtonEventState {
    button: u32,
    button_state: ButtonState,
    tool_type: TabletToolType,
}

impl ButtonEventState {
    pub fn new(button: u32, button_state: ButtonState, tool_type: TabletToolType) -> Self {
        Self {
            button,
            button_state,
        tool_type,
        }
    }
}
