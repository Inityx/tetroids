#![allow(dead_code)]
use ::x11::xlib;
use ::std::mem;
use ::std::ptr;

use game;

struct Renderer {
    display_ptr: *mut xlib::Display,
    window: xlib::Window,
    
    // wm_protocols: xlib::Atom,
    // wm_delete_window: xlib::Atom,
}

impl Renderer {
    pub unsafe fn new() -> Renderer {
        let display_ptr = xlib::XOpenDisplay(ptr::null());
        if display_ptr.is_null() { panic!("Failed to open XDisplay"); }
            
        let screen = xlib::XDefaultScreen(display_ptr);
        let window = xlib::XRootWindow(display_ptr, screen);
        
        Renderer {
            display_ptr: display_ptr,
            window: window
        }
    }
    
    pub fn close(&mut self) {
        
    }
    
    pub unsafe fn tick<EventHandler>(&mut self, event_handler: &mut EventHandler) -> bool
        where EventHandler: FnMut(&xlib::XEvent) {
        
        let mut window_open = true;
        let mut event: xlib::XEvent = mem::uninitialized();
        
        while xlib::XPending(self.display_ptr) > 0 {
            xlib::XNextEvent(self.display_ptr, &mut event);
            
            /*match event.get_type() {
                 xlib::ClientMessage => {
                    let xclient: xlib::XClientMessageEvent = From::from(event);
                    if xclient.message_type == xlib::Atom && xclient.format == 32 {
                        
                    }
                },
                _ => event_handler(&event),
            } */
            
            event_handler(&event)
        }
        
        
        window_open
    }
    
    pub unsafe fn render(&mut self, game: &game::Game) {
        
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.close();
    }
}
