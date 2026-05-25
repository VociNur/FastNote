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

pub struct InputManager {
    pub user_inputs: UserInputs,
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            user_inputs: UserInputs::default(),
        }
    }
}

impl InputManager {
    pub fn manage_events(self: &mut Self, state: &mut State, event: Event) {
        match event {
            egui::Event::Touch {
                device_id: _device_id,
                id,
                phase,
                pos,
                force: _force,
            } => {
                println!("event {:?}", event);
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
                            let finger = &mut self.user_inputs.fingers[finger_id].clone();
                            self.on_finger_move(state, finger, pos);
                            finger.pos = pos;
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
                        self.user_inputs.fingers.retain(|f| {f.id != id});
                    }
                }
            }
            // egui::Event::PointerMoved(pos) => {}
            _ => {
                println!("event {:?}", event);
            }
        }
    }

    pub fn nbr_finger(self: &mut Self) -> usize {
        self.user_inputs.fingers.len()
    }

    pub fn on_finger_start(self: &mut Self, state: &mut State, finger: &mut Finger) {}

    pub fn on_finger_move(self: &mut Self, state: &mut State, start_finger: &Finger, new_pos: Pos2) {
        if self.nbr_finger() == 1{
            
        }
        
    }

    pub fn on_finger_end(self: &mut Self, state: &mut State, start_finger: &Finger, end_pos: Pos2) {}
}
