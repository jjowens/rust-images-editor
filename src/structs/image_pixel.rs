pub struct ImagePixel {
    x: u32,
    y: u32
}

impl ImagePixel {
    pub fn new(x: u32, y: u32) -> ImagePixel {
        ImagePixel {
            x,
            y
        }
    }

    pub fn get_x(&self) -> u32 { self.x }
    pub fn get_y(&self) -> u32 { self.y }
}