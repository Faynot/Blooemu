#[cfg(target_os = "linux")]
use procfs::process::{all_processes, Process};
#[cfg(target_os = "windows")]
use std::ffi::CStr;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};

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
                let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                // Use case-insensitive comparison for Windows
                if process_name.to_lowercase().contains(&task_name.to_lowercase()) {
                    // Close the handle after use, without terminating the process
                    CloseHandle(snapshot);
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot); // Close the handle anyway
        None
    }
}

// Function to get the process name by ID for Linux/MacOS
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_process_name(pid: i32) -> Option<String> {
    match Process::new(pid) {
        Ok(process) => Some(process.stat.comm),
        Err(_) => None,
    }
}

// Function to get the process name by ID for Windows
#[cfg(target_os = "windows")]
pub fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            return None;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                if entry.th32ProcessID == pid {
                    let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                        .to_string_lossy()
                        .into_owned();
                    CloseHandle(snapshot); // Close the handle after use, without terminating the process
                    return Some(process_name);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot); // Close the handle anyway
        None
    }
}
