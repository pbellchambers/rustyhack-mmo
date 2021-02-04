pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
    pub current_x: i32,
    pub current_y: i32,
}

impl Viewport {
    pub fn new(width: u32, height: u32, target_fps: u32) -> Viewport {
        Viewport {
            width,
            height,
            target_fps,
            current_x: 0,
            current_y: 0,
        }
    }
}
