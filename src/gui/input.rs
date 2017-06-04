use ::x11::{xlib, xinput2};
use ::std::os;

// const WINDOW_EVENTS: &[i32] = &[xinput2::XI_KeyPress];

pub enum Key {
    ArrowRight = 114,
    ArrowLeft  = 113,
    ArrowUp    = 111,
    ArrowDown  = 116,
    NumPad0    =  90,
}

impl Key {
    pub fn from(other: i32) -> Option<Self> {
        use self::Key::*;
        match other {
            114 => Some(ArrowRight),
            113 => Some(ArrowLeft),
            111 => Some(ArrowUp),
            116 => Some(ArrowDown),
            90  => Some(NumPad0),
            _ => None,
        }
    }
}

pub unsafe fn select_events(display_ptr: *mut xlib::_XDisplay, window: xlib::Window) {
    let mut mask: [os::raw::c_uchar;1] = [0];
    
    xinput2::XISetMask(&mut mask, xinput2::XI_KeyPress);
    
    let mut input_event_mask = xinput2::XIEventMask {
        deviceid: xinput2::XIAllMasterDevices,
        mask_len: mask.len() as i32,
        mask: mask.as_mut_ptr()
    };
    let events_selected = xinput2::XISelectEvents(
        display_ptr, window,
        &mut input_event_mask,
        1
    );
    if events_selected as u8 != xlib::Success {
        panic!("Failed to select events: {:?}", events_selected);
    }
}
