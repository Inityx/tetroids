use super::xlib;
use super::game::color;
use ::std::os::raw::{c_ushort, c_char, c_ulong};

macro_rules! xcolor {
    ($($x:ident),*) => (
        $(
            const $x: xlib::XColor = xlib::XColor {
                red:   color::named::$x.0 as c_ushort,
                green: color::named::$x.1 as c_ushort,
                blue:  color::named::$x.2 as c_ushort,
                
                pixel: 0 as c_ulong,
                flags: xlib::DoRed | xlib::DoGreen | xlib::DoBlue,
                pad: 0 as c_char,
            };
        )*
    );
}

xcolor!(YELLOW, GREEN, CYAN, BLUE, PURPLE, RED, ORANGE);
