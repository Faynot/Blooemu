use std::env;
#[cfg(target_os = "linux")]
use procfs::process::all_processes;
#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use std::ffi::{CStr, CString};

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

// Linux implementation using procfs
#[cfg(target_os = "linux")]
pub fn get_pid(task_name: &str) -> Option<i32> {
    for process in all_processes().unwrap() {
        if let Ok(proc) = process {
            if proc.stat.comm == task_name {
                return Some(proc.pid);
            }
        }
    }
    None
}

// MacOS implementation (using procfs-like logic)
#[cfg(target_os = "macos")]
pub fn get_pid(task_name: &str) -> Option<i32> {
    for process in all_processes().unwrap() {
        if let Ok(proc) = process {
            if proc.stat.comm == task_name {
                return Some(proc.pid);
            }
        }
    }
    None
}

// Windows implementation using winapi
#[cfg(target_os = "windows")]
pub fn get_pid(task_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            return None;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                // Используем CStr вместо CString
                let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                if process_name.to_lowercase().contains(&task_name.to_lowercase()) {
                    CloseHandle(snapshot); // Закрываем хендл после использования
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot); // Закрываем хендл в любом случае
        None
    }
}
