pub mod popups;
pub mod macros;
pub mod utils;

pub use popups::{alert_message, error_message};
pub use utils::{open, close};



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message() {
        let message = "Test error message";
        error_message(message, "Test Error Title", None);
    }

    #[test]
    fn test_open() {
        let path = "C:/Users/Happy PC/Desktop/test.txt";
        open(path);
    }

    #[test]
    fn test_alert_macro() {
        alert!("sss");
        alert!(
            "Test with yes and no callbacks",
            "Custom Title",
            "yesno",
            || {
                error!("you choose yes");
            },
            || {
                error!("you choose no");
            }
        );
    }

    #[test]
    fn test_error_macro() {
        error!("Test error macro");
        error!("Test error with title", "Critical Error");
        error!("Test error with callback", "Critical Error", || {
            println!("Error callback executed");
        });
    }

    #[test]
    fn test_close() {
        let process_name = "notepad.exe";
        close(process_name);
    }
}
