// #![warn(clippy::all, rust_2018_idioms)]

// use egui::Color32;
pub mod app;
pub mod state;
pub mod themes;
pub mod ui;
pub mod icons;
pub mod pen;
pub mod user_project;
pub mod edition;
pub mod stylet;
pub mod gpu;
pub mod strokes;
pub mod user_file;
pub mod input_manager;
pub mod gpuview;
// pub fn str_hex_to_color(hex: &str)->Color32{
//     let hx = hex.trim_start_matches("#");
//     let r = u8::from_str_radix(&hx[0..2],16).unwrap();
//     let g = u8::from_str_radix(&hx[2..4],16).unwrap();
//     let b = u8::from_str_radix(&hx[4..6],16).unwrap();
//     Color32::from_rgb(r, g, b)
// }
fn get_screen_size() -> (u32, u32) {
    let Ok(output) = std::process::Command::new("xrandr").output() else {
        return (1920, 1200);
    };
    
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.contains(" connected primary") {
            if let Some(res) = line.split_whitespace()
                .find(|s| s.contains('x') && s.contains('+'))
            {
                let parts: Vec<&str> = res.split('x').collect();
                if parts.len() >= 2 {
                    let w = parts[0].parse().unwrap_or(1920);
                    let h = parts[1].split('+').next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1200);
                    return (w, h);
                }
            }
        }
    }
    println!("Error !");
    assert!(false);
    (1,1)
}
