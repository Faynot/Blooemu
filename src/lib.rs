pub mod macros;
pub mod popups;
pub mod utils;
mod process;

pub use popups::{alert_message, error_message};
pub use utils::{close, open};
pub use process::{get_process_name, get_pid};

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
    fn test_get_pid() {
        let task_name = "notepad";
        if let Some(pid) = get_pid(task_name) {
            println!("Found task {} with PID: {}", task_name, pid);
        } else {
            eprintln!("Task {} not found", task_name);
        }
    }

    #[test]
    fn test_alert_macro() {
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
        if let Some(pid) = get_pid("notepad.exe") {
            println!("Found process ID: {}", pid);
            let namepid = get_process_name(pid);
            println!("{:?}", namepid);
        } else {
            println!("Process not found.");
        }
    }
}
