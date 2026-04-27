use egui::Color32;



pub struct Pen{
    pub color: Color32,
    pub size: u32,
    pub erase: bool,
}


pub const DEFAULT_PEN: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1,
    erase: false,
};


pub const DEFAULT_ERASER: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1,
    erase: true,
};

impl Pen{
    pub fn new(color: Color32, size: u32, erase: bool)-> Self{
        Self{
            color,
            size,
            erase
        }
    }
}
