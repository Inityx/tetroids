#![allow(dead_code)]

mod input;
mod xcolor;

use ::x11::{xlib, xinput2};
use ::std::ffi;
use ::std::mem;
use ::std::ptr;
use ::std::os;
use ::std::time;
use ::std::thread;
use ::std::sync;

use game;
use self::input::Key;

const WINDOW_PADDING: i32 = 24;
const WINDOW_WIDTH: os::raw::c_uint = 120 + 2*WINDOW_PADDING as u32;
const WINDOW_HEIGHT: os::raw::c_uint = 240 + 2*WINDOW_PADDING as u32;
const WINDOW_TITLE: &str = "Tetroids";
const INITIAL_TICK_MS: u64 = 1000;

#[derive(Copy, Clone)]
enum DurationOrQuit {
    Dur(time::Duration),
    Quit,
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
    
    pub fn new() -> GUI {
        let display_ptr = unsafe { xlib::XOpenDisplay(ptr::null()) };
        if display_ptr.is_null() { panic!("Failed to open XDisplay"); }
        
        let wm_delete_window = unsafe {
            xlib::XInternAtom(
                display_ptr,
                ffi::CString::new("WM_DELETE_WINDOW").unwrap().as_ptr(),
                xlib::False
            )
        };
        let wm_protocols = unsafe {
            xlib::XInternAtom(
                display_ptr,
                ffi::CString::new("WM_PROTOCOLS").unwrap().as_ptr(),
                xlib::False
            )
        };
        if wm_delete_window == 0 || wm_protocols == 0 { panic!("Failed to load Xlib Atoms."); }
        
        let window = unsafe { self::GUI::initialize_window(display_ptr, wm_delete_window) };
        unsafe { self::input::select_events(display_ptr, window); }

        let gfx_context = unsafe {
            xlib::XCreateGC(
                display_ptr,
                window,
                0u64, // Fixme
                &mut mem::zeroed::<xlib::XGCValues>(),
            )
        };
        
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
    
    pub fn close(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display_ptr, self.window);
            xlib::XCloseDisplay(self.display_ptr);
        }
    }
    
    fn game_try_move(game: &mut game::Game, key: Key) -> Result<(),()> {
        use game::Movement::*;
        let movement = match key {
            Key::ArrowLeft => MoveLeft,
            Key::ArrowRight => MoveRight,
            Key::ArrowDown => MoveDown,
            Key::NumPad0 => RotRight,
            Key::ArrowUp => unreachable!(),
        };
        
        game.try_move_cursor(movement)
    }
    
    fn handle_client_message(&self, event: xlib::XEvent) -> bool {
        let message: xlib::XClientMessageEvent = From::from(event);
        message.message_type != self.wm_protocols ||
            message.format != 32 ||
            message.data.get_long(0) as xlib::Atom != self.wm_delete_window
    }
    
    fn handle_configure_notify(&mut self, event: xlib::XEvent) -> bool {
        let configure_event: xlib::XConfigureEvent = From::from(event);
        
        self.width  = configure_event.width  as os::raw::c_uint;
        self.height = configure_event.height as os::raw::c_uint;
        println!("Resizing to {}x{}.", self.width, self.height);
        true
    }
    
    fn handle_generic_event(
        &mut self, event: xlib::XEvent,
        game: &mut game::Game
    ) -> bool {
        let mut cookie: xlib::XGenericEventCookie = From::from(event);
        
        let data_retrieved = unsafe { xlib::XGetEventData(self.display_ptr, &mut cookie) };
        if data_retrieved == xlib::False {
            panic!("Failed to retrieve xinput event data.");
        }
        
        if cookie.evtype == xinput2::XI_KeyPress {
            let event_data: &xinput2::XIDeviceEvent = unsafe { mem::transmute(cookie.data) };
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
                
                if game.get_cursor().is_none() {
                    game.refill_cursor();
                
                }
                self.render(game);
                
                if game.evaluate_score() {
                    thread::sleep(time::Duration::from_millis(300));
                    game.project_cursor();
                    self.render(game);
                }
            }
        }
        
        true
    }
    
    fn start_timing_thread(
        &self, tick: sync::Arc<sync::Mutex<DurationOrQuit>>
    ) -> thread::JoinHandle<()> {
        let thread_display_ptr = self.display_ptr.clone() as usize;
        let thread_window = self.window.clone();
        
        thread::spawn(move || {
            let mut event: xlib::XKeyPressedEvent = unsafe { mem::uninitialized() };
            // configure event
            
            while let DurationOrQuit::Dur(sleep_time) = *tick.lock().unwrap() {
                thread::sleep(sleep_time);
                unsafe {
                    xlib::XSendEvent(
                        thread_display_ptr as *mut xlib::_XDisplay,
                        thread_window,
                        0,
                        0,
                        (&mut event as *mut xlib::XKeyEvent) as *mut xlib::XEvent,
                    );
                }
            }
        })
    }
    
    pub fn play(&mut self, game: &mut game::Game) {
        unsafe { xlib::XMapWindow(self.display_ptr, self.window); }
        let mut event: xlib::XEvent = unsafe { mem::uninitialized() };
        
        game.refill_cursor();
        self.render(game);
        
        let tick = sync::Arc::new(
            sync::Mutex::new(
                DurationOrQuit::Dur(
                    time::Duration::from_millis(INITIAL_TICK_MS)
                )
            )
        );
        
        // let timing_thread = self.start_timing_thread(tick.clone());
        
        let mut running = true;
        while running {
            unsafe { xlib::XNextEvent(self.display_ptr, &mut event); }
            running = match event.get_type() {
                xlib::ClientMessage   => self.handle_client_message(event),
                xlib::ConfigureNotify => self.handle_configure_notify(event),
                xlib::GenericEvent    => self.handle_generic_event(event, game),
                _ => {
                    println!("Received unhandled event '{}'", event.get_type());
                    true
                },
            };
            
            if let DurationOrQuit::Dur(mut value) = *tick.lock().unwrap() {
                value -= time::Duration::from_millis(10);
            }
        }
        
        //timing_thread.join().unwrap();
    }
    
    pub fn render(&mut self, game: &game::Game) {
        unsafe {
            xlib::XClearWindow(self.display_ptr, self.window);
            // draw border
            xlib::XDrawRectangle(
                self.display_ptr,
                self.window,
                self.gfx_context,
                WINDOW_PADDING/2, WINDOW_PADDING/2,
                (120 + WINDOW_PADDING) as u32,
                (240 + WINDOW_PADDING) as u32,
            );
        }
        
        // draw board
        println!("Rendering Board");
        for (x_index, y_index, square) in game.board_iter_with_index() {
            if let Some(square) = square {
                unsafe {
                    xlib::XFillRectangle(
                        self.display_ptr,
                        self.window,
                        self.gfx_context,
                             12*x_index as i32  + WINDOW_PADDING + 1,
                        228-(12*y_index as i32) + WINDOW_PADDING + 1,
                        10, 10,
                    );
                }
            }
        }
        // draw cursor and projection
        if let Some(ref cursor) = game.get_cursor() {
            println!("Rendering Piece");
            for coord in cursor.real_locations().iter() {
                unsafe {
                    xlib::XFillRectangle(
                        self.display_ptr,
                        self.window,
                        self.gfx_context,
                             12*coord.0 as i32  + WINDOW_PADDING + 1,
                        228-(12*coord.1 as i32) + WINDOW_PADDING + 1,
                        10, 10,
                    );
                }
            }
        }
        // draw projection
        if let Some(ref projection) = game.get_projection() {
            println!("Rendering Projection");
            for coord in projection.real_locations().iter() {
                unsafe {
                    xlib::XFillRectangle(
                        self.display_ptr,
                        self.window,
                        self.gfx_context,
                             12*coord.0 as i32  + WINDOW_PADDING + 1,
                        228-(12*coord.1 as i32) + WINDOW_PADDING + 1,
                        10, 10,
                    );
                }
            }
        }
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        self.close();
    }
}
