#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub mod named {
    use super::Color;
    pub const YELLOW: Color = Color(255, 255,   0);
    pub const GREEN:  Color = Color(  0, 255,   0);
    pub const CYAN:   Color = Color(  0, 255, 255);
    pub const BLUE:   Color = Color(  0,   0, 255);
    pub const PURPLE: Color = Color(127,   0, 255);
    pub const RED:    Color = Color(255,   0,   0);
    pub const ORANGE: Color = Color(255, 127,   0);
}
