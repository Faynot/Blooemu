use std::env;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

pub fn set_timer(seconds: u64, callback: impl FnOnce() + Send + 'static) -> Sender<()> {
    let (tx, rx) = channel(); // Создаем канал для отмены таймера

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(seconds));

        // Проверяем, был ли отменен таймер
        if rx.try_recv().is_err() {
            callback();
        }
    });

    tx // Возвращаем передатчик для отмены таймера
}

pub fn cancel_timer(tx: Sender<()>) {
    // Отправляем сигнал отмены по каналу
    tx.send(()).unwrap();
}


// Function to open a file or application
pub fn open(path: &str) {
    let os = env::consts::OS;

    let mut command = match os {
        "windows" => {
            let mut command = std::process::Command::new("cmd");
            command.args(["/C", path]);
            command
        }
        "linux" | "macos" => {
            let mut command = std::process::Command::new("xdg-open");
            command.arg(path);
            command
        }
        _ => {
            eprintln!("Unsupported operating system: {}", os);
            return;
        }
    };

    match command.spawn() {
        Ok(_) => {}
        Err(err) => eprintln!("Failed to open {}: {}", path, err),
    }
}

// Function to close a process by name
pub fn close(name: &str) {
    let os = env::consts::OS;

    let mut command = match os {
        "windows" => {
            let mut command = std::process::Command::new("taskkill");
            command.args(["/IM", name, "/F"]);
            command
        }
        "linux" | "macos" => {
            let mut command = std::process::Command::new("pkill");
            command.arg(name);
            command
        }
        _ => {
            eprintln!("Unsupported operating system: {}", os);
            return;
        }
    };

    match command.spawn() {
        Ok(_) => println!("Successfully closed {}", name),
        Err(err) => eprintln!("Failed to close {}: {}", name, err),
    }
}

// Linux/MacOS implementation using procfs
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_pid(task_name: &str) -> Option<i32> {
    for process in all_processes().unwrap() {
        if let Ok(proc) = process {
            // Convert task_name to lower case for case-insensitive comparison
            if proc.stat.comm.to_lowercase() == task_name.to_lowercase() {
                return Some(proc.pid); // Return the PID without closing the process
            }
        }
    }
    None
}

