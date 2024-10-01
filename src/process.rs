#[cfg(target_os = "linux")]
use procfs::process::{all_processes, Process, ProcessStat};
#[cfg(target_os = "windows")]
use std::ffi::CStr;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::FILETIME;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::GetProcessTimes;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::OpenProcess;
#[cfg(target_os = "windows")]
use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
#[cfg(target_os = "windows")]
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;

use std::time::Duration;






#[cfg(target_os = "windows")]
pub fn get_all_processes() -> Vec<(u32, String)> {
    let mut processes = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            return processes;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                processes.push((entry.th32ProcessID, process_name));
                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
    }
    processes
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_all_processes() -> Vec<(i32, String)> {
    all_processes()
        .filter_map(|process| {
            let process = process.ok()?;
            Some((process.stat.pid, process.stat.comm))
        })
        .collect()
}



#[cfg(target_os = "windows")]
pub fn get_process_cpu_usage(task_name: &str) -> Option<f32> {
    let pid = get_pid(task_name)?;

    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
        if process_handle.is_null() {
            return None;
        }

        let mut creation_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut exit_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut kernel_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut user_time: FILETIME = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };

        if GetProcessTimes(
            process_handle,
            &mut creation_time,
            &mut exit_time,
            &mut kernel_time,
            &mut user_time,
        ) != 0
        {
            let kernel_time = filetime_to_duration(kernel_time);
            let user_time = filetime_to_duration(user_time);
            let total_time = kernel_time + user_time;

            // Calculate CPU usage as a percentage
            let cpu_usage = total_time.as_secs_f32(); // Adjust as necessary to reflect the total elapsed time since creation
            CloseHandle(process_handle);
            return Some(cpu_usage);
        }

        CloseHandle(process_handle);
        None
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_process_cpu_usage(task_name: &str) -> Option<f32> {
    let pid = get_pid(task_name)?;

    match Process::new(pid) {
        Ok(process) => {
            let cpu_usage = process.stat.utime as f32 + process.stat.stime as f32; // User time + System time
            let total_time =
                process.stat.utime + process.stat.stime + process.stat.cutime + process.stat.cstime;

            if total_time > 0 {
                return Some((cpu_usage / total_time as f32) * 100.0); // CPU usage as a percentage
            }
        }
        Err(_) => return None,
    }

    None
}
#[cfg(target_os = "windows")]
fn filetime_to_duration(filetime: FILETIME) -> Duration {
    let ft = u64::from(filetime.dwHighDateTime) << 32 | u64::from(filetime.dwLowDateTime);
    Duration::from_nanos(ft * 10) // Convert from 100-nanosecond intervals to nanoseconds
}
#[cfg(target_os = "windows")]
pub fn get_process_memory_usage(task_name: &str) -> Option<u64> {
    let pid = get_pid(task_name)?;

    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
        if process_handle.is_null() {
            return None;
        }

        let mut memory_counters: PROCESS_MEMORY_COUNTERS = std::mem::zeroed();
        if GetProcessMemoryInfo(
            process_handle,
            &mut memory_counters,
            size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        ) != 0
        {
            CloseHandle(process_handle); // Closes a handle
            return Some(memory_counters.WorkingSetSize.try_into().unwrap()); // Convert usize to u64
        }

        CloseHandle(process_handle); // Close the handle in case of an error
        None
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_process_memory_usage(task_name: &str) -> Option<u64> {
    let pid = get_pid(task_name)?;

    match Process::new(pid) {
        Ok(process) => {
            let memory_info = process.stat;
            Some(memory_info.rss as u64) // // Return RSS (Resident Set Size) as the amount of memory used
        }
        Err(_) => None,
    }
}


#[cfg(target_os = "windows")]
pub fn get_pid(task_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            return None;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                if process_name
                    .to_lowercase()
                    .contains(&task_name.to_lowercase())
                {
                    CloseHandle(snapshot);
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        None
    }
}


#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_process_name(pid: i32) -> Option<String> {
    match Process::new(pid) {
        Ok(process) => Some(process.stat.comm),
        Err(_) => None,
    }
}


#[cfg(target_os = "windows")]
pub fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            return None;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                if entry.th32ProcessID == pid {
                    let process_name = CStr::from_ptr(entry.szExeFile.as_ptr())
                        .to_string_lossy()
                        .into_owned();
                    CloseHandle(snapshot);
                    return Some(process_name);
                }

                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        None
    }
}
