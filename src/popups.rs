#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{
    MessageBoxW, IDNO, IDOK, IDYES, MB_ICONERROR, MB_ICONWARNING, MB_OK, MB_YESNO,
};

// Function to display a warning window
pub fn alert_message(
    message: &str,
    title: &str,
    buttons: Option<&str>,
    yes_callback: Option<fn()>,
    no_callback: Option<fn()>,
) {
    // Windows logic
    #[cfg(target_os = "windows")]
    {
        let wide_message: Vec<u16> = OsStr::new(message)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let wide_title: Vec<u16> = OsStr::new(title)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let flags = match buttons {
            Some("yesno") => MB_YESNO | MB_ICONWARNING,
            _ => MB_OK,
        };

        let result = unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                wide_message.as_ptr(),
                wide_title.as_ptr(),
                flags,
            )
        };

        match result {
            IDYES => {
                if let Some(yes_fn) = yes_callback {
                    yes_fn();
                }
            }
            IDNO => {
                if let Some(no_fn) = no_callback {
                    no_fn();
                }
            }
            _ => {}
        }
    }

    // Linux logic
    #[cfg(target_os = "linux")]
    {
        use notify_rust::Notification;

        Notification::new()
            .summary(title)
            .body(message)
            .show()
            .expect("Failed to send notification");

        if let Some(yes_fn) = yes_callback {
            yes_fn();
        }
    }

    // Logic for other OS
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        println!("{}: {}", title, message);

        if let Some(yes_fn) = yes_callback {
            yes_fn();
        }
    }
}

// Function to display the error window
pub fn error_message(message: &str, title: &str, callback: Option<fn()>) {
    // Windows logic
    #[cfg(target_os = "windows")]
    {
        let wide_message: Vec<u16> = OsStr::new(message)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let wide_title: Vec<u16> = OsStr::new(title)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let flags = MB_OK | MB_ICONERROR;

        let result = unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                wide_message.as_ptr(),
                wide_title.as_ptr(),
                flags,
            )
        };

        if result == IDOK {
            if let Some(callback_fn) = callback {
                callback_fn();
            }
        }
    }

    // Linux logic
    #[cfg(target_os = "linux")]
    {
        use notify_rust::Notification;

        Notification::new()
            .summary(title)
            .body(message)
            .urgency(notify_rust::Urgency::Critical)
            .show()
            .expect("Failed to send error notification");

        if let Some(callback_fn) = callback {
            callback_fn();
        }
    }

    // Logic for other OS
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        eprintln!("{}: {}", title, message);

        if let Some(callback_fn) = callback {
            callback_fn();
        }
    }
}
