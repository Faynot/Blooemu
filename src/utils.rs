use std::env;
use std::process::Command;

pub fn open(path: &str) {
    let os = env::consts::OS;

    let mut command = match os {
        "windows" => {
            let mut command = Command::new("cmd");
            command.args(["/C", path]);
            command
        }
        "linux" | "macos" => {
            let mut command = Command::new("xdg-open");
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

pub fn close(name: &str) {
    let os = env::consts::OS;

    let mut command = match os {
        "windows" => {
            let mut command = Command::new("taskkill");
            command.args(["/IM", name, "/F"]);
            command
        }
        "linux" | "macos" => {
            let mut command = Command::new("pkill");
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
