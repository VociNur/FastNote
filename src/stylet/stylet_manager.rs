use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use eframe::egui::{self, Context, Pos2, Rect};
use input::event::{
    pointer::ButtonState,
    tablet_tool::{ProximityState, TabletToolType, TipState},
};

use crate::{state::State, strokes::strokes::StrokePoint, stylet::stylet::StyletState};

#[derive(Default)]
pub struct StyletManager {
    pub stylet: StyletState,
    pub events: Arc<Mutex<Vec<StyletEvent>>>, //un peu galère pas de trait copy
    pub last_erase_at_pos: Option<Pos2>,
}

impl StyletManager {
    pub fn manage_events(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        _has_focus: &bool,
        gpu_rect: &Option<Rect>,
    ) {
        let events = std::mem::take(&mut *self.events.lock().unwrap());
        for event in events {
            match event {
                StyletEvent::Axis(axis_event_state) => {
                    self.on_axis_event(ctx, state, &axis_event_state, gpu_rect);
                    self.stylet.pos = axis_event_state.pos;
                    self.stylet.pressure = axis_event_state.pressure;
                    self.stylet.distance = axis_event_state.distance;
                    self.stylet.tilt_x = axis_event_state.tilt_x;
                    self.stylet.tilt_y = axis_event_state.tilt_y;
                    self.stylet.tool_type = axis_event_state.tool_type;
                }
                StyletEvent::Tip(tip_event_state) => {
                    self.on_tip_event(ctx, state, &tip_event_state, gpu_rect);
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
    pub fn touch_gpu(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        pos: Pos2,
        pressure: f64,
        tool_type: TabletToolType,
        opt_gpu_rect: &Option<Rect>,
    ) {
        if !self.stylet.pressed {
            return;
        }
        // println!("test");
        // if state.current_file.is_none() {
        //     state.current_file = Some(UserFile::new(PathBuf::from("")));
        // }
        if opt_gpu_rect.is_none() {
            // println!("Gpu rect is none\n Return\n");
            return;
        }
        let gpu_rect = opt_gpu_rect.unwrap();
        // println!("gpu rect {:?} {:?}", gpu_rect, pos);
        if gpu_rect.contains(pos) {
            if let Some(file) = state.current_file.as_mut() {
                let draw_pos =
                    (pos - gpu_rect.min) / state.gpu_view.zoom + state.gpu_view.top_left.to_vec2();
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
                    file.erase_at(draw_pos.to_pos2(), 1f32);
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
        opt_gpu_rect: &Option<Rect>,
    ) {
        // println!("axis: {axis_event_state:?}");
        self.touch_gpu(
            ctx,
            state,
            axis_event_state.pos,
            axis_event_state.pressure,
            axis_event_state.tool_type,
            opt_gpu_rect,
        );
    }

    pub fn on_tip_event(
        self: &mut Self,
        ctx: &Context,
        state: &mut State,
        tip_event_state: &TipEventState,
        opt_gpu_rect: &Option<Rect>,
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
                opt_gpu_rect,
            );
        } else {
            state.cursor_icon = egui::CursorIcon::Default;
            if let Some(file) = &mut state.current_file {
                file.save_current_stroke(&state.color_palette.pen);
            }
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
