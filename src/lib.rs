// Импорт winapi только для Windows
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{MessageBoxW, MB_OK};

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

// Функция для отображения окна с предупреждением
pub fn alert_message(message: &str) {
    // Логика для Windows
    #[cfg(target_os = "windows")]
    {
        let wide: Vec<u16> = OsStr::new(message)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            MessageBoxW(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
        }
    }

    // Логика для Linux
    #[cfg(target_os = "linux")]
    {
        use notify_rust::Notification;

        Notification::new()
            .summary("Alert")
            .body(message)
            .show()
            .expect("Failed to send notification");
    }

    // Запасной вариант для других ОС
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        println!("Alert: {}", message);
    }
}

// Макрос alert
#[macro_export]
macro_rules! alert {
    ($msg:expr) => {
        $crate::alert_message($msg);
    };
}
