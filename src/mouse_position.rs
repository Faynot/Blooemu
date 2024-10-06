pub struct Position {
    pub x: i32,
    pub y: i32
}

/// Mouse Position enum, which can either be a position or an error.
pub enum Mouse {
    Position { x: i32, y: i32 },
    Error
}

#[cfg(target_os = "windows")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use winapi::{um::winuser::{GetCursorPos}, shared::windef::POINT};

        let mut point = POINT { x: 0, y: 0 };
        let result = unsafe { GetCursorPos(&mut point) };

        if result == 1 {
            return Mouse::Position { x: point.x, y: point.y }
        }

        Mouse::Error
    }
}

#[cfg(target_os = "macos")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use core_graphics::event::{CGEvent};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let event = CGEvent::new(CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap());
        let point = match event {
            Ok(event) => {
                let point = event.location();
                Mouse::Position { x: point.x as i32, y: point.y as i32 }
            },
            Err(_) => return Mouse::Error,
        };

        point
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use x11_dl::xlib;

        // Initialize Xlib and open a connection to the X server
        let xlib = xlib::Xlib::open().unwrap();
        let display = unsafe { (xlib.XOpenDisplay)(std::ptr::null()) };

        // Get the root window for the current screen
        let screen = unsafe { (xlib.XDefaultScreen)(display) };
        let root = unsafe { (xlib.XRootWindow)(display, screen) };

        // Get the pointer position
        let mut root_return: xlib::Window = 0;
        let mut child_return: xlib::Window = 0;
        let mut root_x_return: i32 = -1;
        let mut root_y_return: i32 = -1;
        let mut win_x_return: i32 = 0;
        let mut win_y_return: i32 = 0;
        let mut mask_return: u32 = 0;

        unsafe {
            (xlib.XQueryPointer)(
                display,
                root,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            );
        }

        // Close the connection to the X server
        unsafe {
            (xlib.XCloseDisplay)(display);
        }

        if root_x_return == -1 || root_y_return == -1 {
            return Mouse::Error;
        }

        Mouse::Position {
            x: root_x_return,
            y: root_y_return,
        }
    }
}