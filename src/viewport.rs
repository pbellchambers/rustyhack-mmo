pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32, target_fps: u32) -> Viewport {
        Viewport {
            width,
            height,
            target_fps,
        }
    }
}
