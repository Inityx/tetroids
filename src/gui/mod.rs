#![allow(dead_code)]

mod xcolor;

use ::x11::{xlib, xinput2};
use ::std::ffi;
use ::std::mem;
use ::std::ptr;
use ::std::os;

use game;

const WINDOW_WIDTH: os::raw::c_uint = 1280;
const WINDOW_HEIGHT: os::raw::c_uint = 720;
const WINDOW_TITLE: &str = "Tetroids";
const WINDOW_EVENTS: &[i32] = &[xinput2::XI_KeyPress];

enum Key {
    ArrowRight = 114,
    ArrowLeft  = 113,
    ArrowUp    = 111,
    ArrowDown  = 116,
    NumPad0    =  90,
}

impl Key {
    fn from(other: i32) -> Option<Self> {
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

pub struct GUI {
    display_ptr: *mut xlib::Display,
    window: xlib::Window,
    gfx_context: xlib::GC,
    
    width: os::raw::c_uint,
    height: os::raw::c_uint,
    
    wm_delete_window: xlib::Atom,
    wm_protocols: xlib::Atom,
}

impl GUI {
    unsafe fn initialize_window(
        display_ptr: *mut xlib::_XDisplay,
        mut wm_delete_window: xlib::Atom,
    ) -> xlib::Window {
        
        let screen_num = xlib::XDefaultScreen(display_ptr);
        let root       = xlib::XRootWindow   (display_ptr, screen_num);
        
        let mut attributes: xlib::XSetWindowAttributes = mem::zeroed();
        attributes.background_pixel = xlib::XWhitePixel(display_ptr, screen_num);
        
        let window = xlib::XCreateWindow(
            display_ptr, root, 0, 0,
            WINDOW_WIDTH, WINDOW_HEIGHT, 0, 0,
            xlib::InputOutput as os::raw::c_uint,
            ptr::null_mut(),
            xlib::CWBackPixel,
            &mut attributes,
        );
        
        xlib::XStoreName(
            display_ptr,
            window,
            ffi::CString::new(WINDOW_TITLE).unwrap().as_ptr(),
        );
        
        let protocols_set = xlib::XSetWMProtocols(
            display_ptr,
            window,
            &mut wm_delete_window as *mut xlib::Atom,
            1,
        );
        if protocols_set == xlib::False { panic!("Failed to set WM protocols."); }
        
        window
    }
    
    unsafe fn select_events(
        display_ptr: *mut xlib::_XDisplay,
        window: xlib::Window,
    ) {
        let mut mask: [os::raw::c_uchar;1] = [0];
        
        for &event in WINDOW_EVENTS {
            xinput2::XISetMask(&mut mask, event);
        }
        
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
    
    pub unsafe fn new() -> GUI {
        let display_ptr = xlib::XOpenDisplay(ptr::null());
        if display_ptr.is_null() { panic!("Failed to open XDisplay"); }
        
        let wm_delete_window = xlib::XInternAtom(
            display_ptr,
            ffi::CString::new("WM_DELETE_WINDOW").unwrap().as_ptr(),
            xlib::False
        );
        let wm_protocols = xlib::XInternAtom(
            display_ptr,
            ffi::CString::new("WM_PROTOCOLS").unwrap().as_ptr(),
            xlib::False
        );
        if wm_delete_window == 0 || wm_protocols == 0 { panic!("Failed to load Xlib Atoms."); }
        
        let window = self::GUI::initialize_window(display_ptr, wm_delete_window);
        self::GUI::select_events(display_ptr, window);

        let gfx_context = xlib::XCreateGC(
            display_ptr,
            window,
            0u64, // Fixme
            &mut mem::zeroed::<xlib::XGCValues>(),
        );
        
        GUI {
            display_ptr: display_ptr,
            window: window,
            gfx_context: gfx_context,
            
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            
            wm_delete_window: wm_delete_window,
            wm_protocols: wm_protocols,
        }
    }
    
    pub unsafe fn close(&mut self) {
        xlib::XDestroyWindow(self.display_ptr, self.window);
        xlib::XCloseDisplay(self.display_ptr);
    }
    
    fn game_try_move(game: &mut game::Game, key: Key) -> Result<(),()> {
        use game::Movement::*;
        let movement = match key {
            Key::ArrowLeft => MoveLeft,
            Key::ArrowRight => MoveRight,
            Key::ArrowDown => MoveDown,
            Key::NumPad0 => RotRight,
            Key::ArrowUp => return Err(()),
        };
        
        game.try_move_cursor(movement)
    }
    
    fn handle_client_message(&self, event: xlib::XEvent) -> bool {
        let message: xlib::XClientMessageEvent = From::from(event);
        message.message_type != self.wm_protocols ||
            message.format != 32 ||
            message.data.get_long(0) as xlib::Atom != self.wm_delete_window
    }
    
    unsafe fn handle_configure_notify(&mut self, event: xlib::XEvent) -> bool {
        let configure_event: xlib::XConfigureEvent = From::from(event);
        
        self.width  = configure_event.width  as os::raw::c_uint;
        self.height = configure_event.height as os::raw::c_uint;
        true
    }
    
    unsafe fn handle_generic_event(&mut self, event: xlib::XEvent, game: &mut game::Game) -> bool {
        let mut cookie: xlib::XGenericEventCookie = From::from(event);
        
        let data_retrieved = xlib::XGetEventData(self.display_ptr, &mut cookie);
        if data_retrieved == xlib::False {
            panic!("Failed to retrieve xinput event data.");
        }
        
        if cookie.evtype == xinput2::XI_KeyPress {
            let event_data: &xinput2::XIDeviceEvent = mem::transmute(cookie.data);
            if let Some(key) = Key::from(event_data.detail) {
                let result = match key {
                    Key::ArrowRight |
                    Key::ArrowLeft |
                    Key::ArrowDown |
                    Key::NumPad0 => self::GUI::game_try_move(game, key),
                    Key::ArrowUp => {
                        game.place_cursor();
                        Ok(())
                    },
                };
                self.render(game);
            }
        }
        
        true
    }
    
    pub unsafe fn play(&mut self, game: &mut game::Game) {
        xlib::XMapWindow(self.display_ptr, self.window);
        let mut event: xlib::XEvent = mem::uninitialized();
        
        game.refill_cursor();
        
        self.render(game);
        let mut running = true;
        while running {
            xlib::XNextEvent(self.display_ptr, &mut event);
            running = match event.get_type() {
                xlib::ClientMessage   => self.handle_client_message(event),
                xlib::ConfigureNotify => self.handle_configure_notify(event),
                xlib::GenericEvent    => self.handle_generic_event(event, game),
                _ => true,
            };
        }
    }
    
    pub unsafe fn render(&mut self, game: &game::Game) {
        xlib::XClearWindow(self.display_ptr, self.window);
        xlib::XFillRectangle(
            self.display_ptr,
            self.window,
            self.gfx_context,
            10, 20,
            30, 40,
        );
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        unsafe { self.close(); }
    }
}