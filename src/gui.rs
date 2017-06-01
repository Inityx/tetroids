#![allow(dead_code)]
use ::x11::xlib;
use ::std::ffi;
use ::std::mem;
use ::std::ptr;
use ::std::os;

use game::Game;

const WINDOW_WIDTH: os::raw::c_uint = 1280;
const WINDOW_HEIGHT: os::raw::c_uint = 720;
const WINDOW_TITLE: &str = "Title goes here";

pub struct GUI {
    display_ptr: *mut xlib::Display,
    window: xlib::Window,
    wm_delete_window: xlib::Atom,
    wm_protocols: xlib::Atom,
}

impl GUI {
    unsafe fn initialize_window(
        display_ptr: *mut xlib::_XDisplay,
        mut wm_delete_window: xlib::Atom
    ) -> xlib::Window {
        
        let screen_num = xlib::XDefaultScreen(display_ptr);
        let root       = xlib::XRootWindow   (display_ptr, screen_num);
        
        let mut attributes: xlib::XSetWindowAttributes = mem::zeroed();
        attributes.background_pixel = xlib::XBlackPixel(display_ptr, screen_num);
        
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
            ffi::CString::new(WINDOW_TITLE).unwrap().as_ptr() as *mut os::raw::c_char,
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
        
        GUI {
            display_ptr: display_ptr,
            window: window,
            
            wm_delete_window: wm_delete_window,
            wm_protocols: wm_protocols,
        }
    }
    
    pub unsafe fn close(&mut self) {
        xlib::XDestroyWindow(self.display_ptr, self.window);
        xlib::XCloseDisplay(self.display_ptr);
    }
    
    pub unsafe fn play(&mut self, game: &mut Game) {
        let mut event: xlib::XEvent = mem::uninitialized();
        
        loop {
            xlib::XNextEvent(self.display_ptr, &mut event);
            
            /*match event.get_type() {
                 xlib::ClientMessage => {
                    let xclient: xlib::XClientMessageEvent = From::from(event);
                    if xclient.message_type == xlib::Atom && xclient.format == 32 {
                        
                    }
                },
                _ => event_handler(&event),
            } */
            self.render(game);
        }
    }
    
    pub unsafe fn render(&mut self, game: &Game) {
        
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        unsafe { self.close(); }
    }
}
