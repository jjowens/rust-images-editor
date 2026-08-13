pub struct BlindRow {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl BlindRow {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> BlindRow {
        BlindRow {
            x,
            y,
            width,
            height,
        }
    }

}