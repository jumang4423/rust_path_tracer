
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    // constructor
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue
        }
    }
}