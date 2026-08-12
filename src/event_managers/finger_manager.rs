use eframe::egui::{self, Event, Pos2, TouchId};

use crate::state::State;

#[derive(Clone)]
pub struct Finger {
    pub id: TouchId,
    pub pos: Pos2,
}

pub struct UserInputs {
    pub fingers: Vec<Finger>,
}

impl Default for UserInputs {
    fn default() -> Self {
        Self { fingers: vec![] }
    }
}

pub struct FingerManager {
    pub user_inputs: UserInputs,
}

impl Default for FingerManager {
    fn default() -> Self {
        Self {
            user_inputs: UserInputs::default(),
        }
    }
}

impl FingerManager {
    pub fn manage_events(self: &mut Self, state: &mut State, event: Event, ppp: f32) {
        match event {
            egui::Event::Touch {
                device_id: _device_id,
                id,
                phase,
                pos,
                force: _force,
            } => {
                // println!("event {:?}", event);
                match phase {
                    egui::TouchPhase::Start => {
                        let mut finger = Finger { id, pos };
                        self.on_finger_start(state, &mut finger);
                        self.user_inputs.fingers.push(finger);
                    }

                    egui::TouchPhase::Move => {
                        let opt_finger_id =
                            self.user_inputs.fingers.iter().position(|f| f.id == id);
                        if let Some(finger_id) = opt_finger_id {
                            // let finger = &mut self.user_inputs.fingers[finger_id].clone();
                            self.on_finger_move(state, finger_id, pos, ppp);
                            self.user_inputs.fingers[finger_id].pos = pos;
                        }
                    }

                    egui::TouchPhase::End | egui::TouchPhase::Cancel => {
                        // println!("Touchphase cancel ?? {:?}", event);
                        let opt_finger_id =
                            self.user_inputs.fingers.iter().position(|f| f.id == id);
                        if let Some(finger_id) = opt_finger_id {
                            let finger = &self.user_inputs.fingers[finger_id].clone();
                            self.on_finger_end(state, finger, pos);
                        }
                        self.user_inputs.fingers.retain(|f| f.id != id);
                    }
                }
            }
            // egui::Event::PointerMoved(pos) => {}
            _ => {
                // println!("event {:?}", event);
            }
        }
    }

    pub fn nbr_finger(self: &mut Self) -> usize {
        self.user_inputs.fingers.len()
    }

    pub fn on_finger_start(self: &mut Self, state: &mut State, finger: &mut Finger) {}

    pub fn on_finger_move(
        self: &mut Self,
        state: &mut State,
        finger_id: usize,
        new_pos: Pos2,
        ppp: f32,
    ) {
        let last_finger = &self.user_inputs.fingers[finger_id].clone();
        if self.nbr_finger() == 1 {
            // println!("delta: {}", new_pos - last_finger.pos);
            state.gpu_view.top_left -= (new_pos - last_finger.pos) / state.gpu_view.zoom * ppp;
        }

        if self.nbr_finger() == 2 {
            let other = &self.user_inputs.fingers[1 - finger_id]; // l'autre doigt

            let old_dist = (last_finger.pos - other.pos).length();
            let new_dist = (new_pos - other.pos).length();

            let scale = new_dist / old_dist;
            // Point central entre les deux doigts → centre du zoom
            let center = (new_pos + other.pos.to_vec2()) / 2.0 / state.gpu_view.zoom;
            // Zoom centré sur le point central
            //Le other nous sert de repère
            state.gpu_view.zoom *= scale;
            state.gpu_view.zoom = state.gpu_view.zoom.clamp(1.0, 20.0);
            let last_center = (last_finger.pos + other.pos.to_vec2()) / 2.0 / state.gpu_view.zoom;
            state.gpu_view.top_left += center - last_center;
        }
    }

    pub fn on_finger_end(self: &mut Self, state: &mut State, last_finger: &Finger, end_pos: Pos2) {
        // state.gpu_view.top_left = Pos2 { x: 0f32, y: 0f32 };
        // println!("Actual zoom: {}", state.gpu_view.zoom);
    }
}
