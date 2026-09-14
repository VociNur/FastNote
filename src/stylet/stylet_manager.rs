use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use eframe::egui::{self, Context, Pos2};
use input::event::{
    pointer::ButtonState,
    tablet_tool::{ProximityState, TabletToolType, TipState},
};

use crate::{state::State, strokes::strokes::StrokePoint, stylet::stylet::StyletState};

#[derive(Default)]
pub struct StyletManager {
    pub stylet: StyletState,
    pub events: Arc<Mutex<Vec<MyLibInputEvent>>>, //un peu galère pas de trait copy
    pub last_erase_at_pos: Option<Pos2>,
}

impl StyletManager {
    pub fn manage_events(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        _has_focus: &bool,
    ) {
        let events = std::mem::take(&mut *self.events.lock().unwrap());
        for event in events {
            match event {
                MyLibInputEvent::Stylet(stylet_event)=>{
                    self.manage_stylet_events(ctx, state, _has_focus, stylet_event);
                }
                MyLibInputEvent::Touchpad(touchpad_event)=>{
                    self.manage_touchpad_events(ctx, state, _has_focus, touchpad_event);
                }
             }

        }
    }


    pub fn manage_touchpad_events(&mut self,

        _ctx: &Context,
        state: &mut State,
        _has_focus: &bool,
         event: TouchpadEvent){
         match event
         {
            TouchpadEvent::Zoom(zoom_event_state) => {
                #[cfg(feature = "debug-input")]
                println!("zoom {:?}", zoom_event_state);
                match zoom_event_state.phase{
                    PhaseZoomEventState::BEGIN => {
                        state.gpu_view.last_touchpad_zoom = Some(1.);
                    }
                    PhaseZoomEventState::UPDATE => {
                        state.gpu_view.mult_zoom( zoom_event_state.scale as f32 / state.gpu_view.last_touchpad_zoom.unwrap_or_else(|| {println!("Error last zoom touchpad not init");1.}));
                        state.gpu_view.last_touchpad_zoom = Some(zoom_event_state.scale as f32);
                        
                    }
                    PhaseZoomEventState::END => {
                        state.gpu_view.last_touchpad_zoom = None;                        
                    }

                }                
            },
            // top (-) to bot (+)
            // left (-) to right (+)
            TouchpadEvent::Axis(axis_event_state) => {
                #[cfg(feature = "debug-input")]
                println!("axis {:?}", axis_event_state);
                state.gpu_view.top_left.x -= axis_event_state.dy as f32 * state.touchpad_scalor_settings_x;
                state.gpu_view.top_left.y -= axis_event_state.dx as f32 * state.touchpad_scalor_settings_y;
                
            },
            TouchpadEvent::Move(_move_event_state) => {
                #[cfg(feature = "debug-input")]
                println!("move {:?}", move_event_state);
                
            },
        }
     }
    pub fn manage_stylet_events(&mut self,

        ctx: &Context,
        state: &mut State,
        _has_focus: &bool,
         event: StyletEvent){
        match event{

                StyletEvent::Axis(axis_event_state) => {
                    self.on_axis_event(ctx, state, &axis_event_state);
                    self.stylet.pos = axis_event_state.pos;
                    self.stylet.pressure = axis_event_state.pressure;
                    self.stylet.distance = axis_event_state.distance;
                    self.stylet.tilt_x = axis_event_state.tilt_x;
                    self.stylet.tilt_y = axis_event_state.tilt_y;
                    self.stylet.tool_type = axis_event_state.tool_type;
                }
                StyletEvent::Tip(tip_event_state) => {
                    self.on_tip_event(ctx, state, &tip_event_state);
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
    pub fn touch_gpu(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        pos: Pos2,
        pressure: f64,
        tool_type: TabletToolType,
    ) {
        if !self.stylet.pressed {
            return;
        }
        // println!("test");
        // if state.current_file.is_none() {
        //     state.current_file = Some(UserFile::new(PathBuf::from("")));
        // }
        if state.gpu_rect.is_none() {
            // println!("Gpu rect is none\n Return\n");
            return;
        }
        let gpu_rect = state.gpu_rect.unwrap();
        // println!("gpu rect {:?} {:?}", gpu_rect, pos);
        if gpu_rect.contains(pos) {
            if let Some(file) = state.loaded_page.as_mut() {
                let draw_pos =
                    (pos - gpu_rect.min) / state.gpu_view.get_zoom() + state.gpu_view.top_left.to_vec2();
                let stroke_point = StrokePoint::new(draw_pos.to_pos2(), pressure);
                if tool_type == TabletToolType::Pen {
                    file.add_stroke_point(stroke_point);
                    ctx.request_repaint_after_secs(0.1);
                    self.last_erase_at_pos = None;
                } else if tool_type == TabletToolType::Eraser {
                    let t = Instant::now();
                    let epsilon_sq = 4.;
                    if let Some(last_erase_pos) = self.last_erase_at_pos && last_erase_pos.distance_sq(draw_pos.to_pos2()) < epsilon_sq{
                        println!("erase skip");
                        return;
                    }
                    file.erase_at(draw_pos.to_pos2(), 2f32);
                    let elapsed = t.elapsed();
                    self.last_erase_at_pos = Some(draw_pos.to_pos2());
                    println!("Eraser timing: {:?}", elapsed);
                } else {
                    println!("Tool type not defined");
                    self.last_erase_at_pos = None;
                }
            }
        } else {
            // println!("Outside the gpu rect {:?} {:?}", gpu_rect, pos);
            println!("Outside");
        }
    }

    pub fn on_axis_event(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        axis_event_state: &AxisEventState,
    ) {
        // println!("axis: {axis_event_state:?}");
        self.touch_gpu(
            ctx,
            state,
            axis_event_state.pos,
            axis_event_state.pressure,
            axis_event_state.tool_type,
        );
    }

    pub fn on_tip_event(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        tip_event_state: &TipEventState,
    ) {
        // println!("tip: {tip_event_state:?}");
        //
        if tip_event_state.tip_state == TipState::Down {
            state.cursor_icon = egui::CursorIcon::None;
            self.touch_gpu(
                ctx,
                state,
                tip_event_state.pos,
                tip_event_state.pressure,
                tip_event_state.tool_type,
            );
        } else {
            state.cursor_icon = egui::CursorIcon::Default;
            // if leiit Some(file) = &mut state.current_file {
            //     file.save_current_stroke(&state.color_palette.pen);
            // }
            state.save_visible_strokes();
        }
    }

    pub fn on_proximity_event(self: &mut Self, _proximity_event_state: &ProximityEventState) {
        // println!("proximity: {proximity_event_state:?}");
    }

    pub fn on_button_event(self: &mut Self, _button_event_state: &ButtonEventState) {
        // println!("button: {button_event_state:?}");
    }
}

//ENUM
pub enum MyLibInputEvent{
    Stylet(StyletEvent),
    Touchpad(TouchpadEvent),
}
pub enum StyletEvent {
    Axis(AxisEventState),
    Tip(TipEventState),
    Proximity(ProximityEventState),
    Button(ButtonEventState),
}

pub enum TouchpadEvent {
    Zoom(TouchpadZoomEventState),
    Axis(TouchpadAxisEventState),
    Move(TouchpadMoveEventState),
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
    _button: u32,
    _button_state: ButtonState,
    tool_type: TabletToolType,
}

impl ButtonEventState {
    pub fn new(_button: u32, _button_state: ButtonState, tool_type: TabletToolType) -> Self {
        Self {
            _button,
            _button_state,
            tool_type,
        }
    }
}

#[derive(Debug)]
pub enum PhaseZoomEventState{
    BEGIN, UPDATE, END
}

#[derive(Debug)]
pub struct TouchpadZoomEventState{
    phase: PhaseZoomEventState,
    dx: f64,
    dy: f64,
    scale: f64,
}

impl TouchpadZoomEventState{
    pub fn new(phase: PhaseZoomEventState, dx: f64, dy: f64, scale: f64)->Self{
        Self{
            phase, dx, dy, scale,
        }
    }
}

#[derive(Debug)]
pub struct TouchpadAxisEventState{
    dx: f64,
    dy: f64,
}

impl TouchpadAxisEventState{
    pub fn new(dx: f64, dy: f64)->Self{
            Self{
            dx, dy
        }
    }
}

#[derive(Debug)]
pub struct TouchpadMoveEventState{
    dx: f64,
    dy: f64,
}

impl TouchpadMoveEventState{
    pub fn new(dx: f64, dy: f64)->Self{
            Self{
            dx, dy
        }
    }
}

