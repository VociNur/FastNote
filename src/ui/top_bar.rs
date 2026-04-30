use crate::{app::App, state::State, ui::{menu::draw_menu, ribbon::draw_ribbon}};

pub fn draw_top_bar(ui: &mut egui::Ui, app: &mut App){
    
    egui::Panel::top("top_panel").show_inside(ui, |ui| {//.frame(egui::Frame{fill: Color32::fromrgb(255, 0, 0), ..Default..default()})
        draw_menu(ui, app);
        ui.add_space(app.state.theme.space_between_menu_and_ribbon);
        draw_ribbon(ui, app);
    });
}



