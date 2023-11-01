pub trait Colored {
    pub fn set_background_color(&mut self, color: (u8, u8, u8, u8));

    pub fn get_background_color(&mut self) -> (u8, u8, u8, u8);

    pub fn set_stroke_color(&mut self, color: (u8, u8, u8, u8));

    pub fn get_stroke_color(&mut self) -> (u8, u8, u8, u8);

    pub fn set_stroke_width(&mut self, width: u32);

    pub fn get_stroke_width(&mut self) -> u32;
}