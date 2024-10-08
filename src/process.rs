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
#[allow(dead_code)]
pub fn elevate_privileges_by_pid(pid: u32) -> bool {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{OpenProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::AdjustTokenPrivileges;
    use winapi::um::winbase::LookupPrivilegeValueA;
    use winapi::um::winnt::{
        HANDLE, PROCESS_QUERY_INFORMATION, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
        TOKEN_PRIVILEGES, TOKEN_QUERY,
    };

    unsafe {
        // Открываем процесс по PID
        let process_handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);

        if process_handle.is_null() {
            eprintln!(
                "Failed to open process with PID: {}, error: {}",
                pid,
                GetLastError()
            );
            return false;
        }

        // Открываем токен процесса
        let mut token_handle: HANDLE = null_mut();
        if OpenProcessToken(
            process_handle,
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            eprintln!("Failed to open process token, error: {}", GetLastError());
            CloseHandle(process_handle);
            return false;
        }

        // Получаем LUID для привилегии SeDebugPrivilege
        let priv_name = CString::new("SeDebugPrivilege").unwrap();
        let mut luid = std::mem::zeroed();
        if LookupPrivilegeValueA(null_mut(), priv_name.as_ptr(), &mut luid) == 0 {
            eprintln!(
                "Failed to lookup privilege value, error: {}",
                GetLastError()
            );
            CloseHandle(token_handle);
            CloseHandle(process_handle);
            return false;
        }

        // Настраиваем привилегии
        let mut tp: TOKEN_PRIVILEGES = std::mem::zeroed();
        tp.PrivilegeCount = 1;
        tp.Privileges[0].Luid = luid;
        tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

        if AdjustTokenPrivileges(
            token_handle,
            0,
            &mut tp,
            size_of::<TOKEN_PRIVILEGES>() as u32,
            null_mut(),
            null_mut(),
        ) == 0
        {
            eprintln!(
                "Failed to adjust token privileges, error: {}",
                GetLastError()
            );
            CloseHandle(token_handle);
            CloseHandle(process_handle);
            return false;
        }

        // Закрываем дескрипторы
        CloseHandle(token_handle);
        CloseHandle(process_handle);
        println!("Privileges elevated successfully for PID: {}", pid);
        true
    }
}

#[cfg(target_os = "linux")]
pub fn elevate_privileges_by_pid(pid: u32) -> bool {
    use std::process::Command;

    let output = Command::new("sudo")
        .arg("prctl")
        .arg("--set-privileges")
        .arg(pid.to_string())
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            println!("Privileges elevated for process with PID: {}", pid);
            return true;
        } else {
            eprintln!(
                "Failed to elevate privileges, error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return false;
        }
    }

    eprintln!("Failed to run sudo command.");
    false
}

#[cfg(target_os = "macos")]
pub fn elevate_privileges_by_pid(pid: u32) -> bool {
    use std::process::Command;

    let output = Command::new("sudo")
        .arg("kill")
        .arg("-CONT")
        .arg(pid.to_string())
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            println!("Privileges elevated for process with PID: {}", pid);
            return true;
        } else {
            eprintln!(
                "Failed to elevate privileges, error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return false;
        }
    }

    eprintln!("Failed to run sudo command.");
    false
}

#[cfg(target_os = "windows")]
pub fn elevate_privileges(process_name: &str) -> bool {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::processthreadsapi::{OpenProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::AdjustTokenPrivileges;
    use winapi::um::tlhelp32::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    };
    use winapi::um::winbase::LookupPrivilegeValueA;
    use winapi::um::winnt::{
        HANDLE, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
        TOKEN_PRIVILEGES, TOKEN_QUERY,
    };

    // Find process by name
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == null_mut() {
            eprintln!("Failed to create process snapshot");
            return false;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        let mut process_id: u32 = 0;
        if Process32First(snapshot, &mut entry) == 1 {
            loop {
                let process_name_cstr = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                if process_name_cstr.to_lowercase() == process_name.to_lowercase() {
                    process_id = entry.th32ProcessID;
                    break;
                }
                if Process32Next(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        if process_id == 0 {
            eprintln!("Failed to find process: {}", process_name);
            return false;
        }

        // Open process and elevate privileges
        let process_handle = OpenProcess(TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, 0, process_id);
        if process_handle.is_null() {
            eprintln!("Failed to open process with PID: {}", process_id);
            return false;
        }

        let mut token_handle: HANDLE = null_mut();
        let mut luid = std::mem::zeroed();

        if OpenProcessToken(
            process_handle,
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token_handle,
        ) == 0
        {
            eprintln!("Failed to open process token, error: {}", GetLastError());
            CloseHandle(process_handle);
            return false;
        }

        let privilege_name = CString::new("SeDebugPrivilege").expect("CString::new failed");
        if LookupPrivilegeValueA(null_mut(), privilege_name.as_ptr(), &mut luid) == 0 {
            eprintln!(
                "Failed to lookup privilege value, error: {}",
                GetLastError()
            );
            CloseHandle(process_handle);
            return false;
        }

        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        // Attempt to adjust token privileges
        if AdjustTokenPrivileges(
            token_handle,
            0,
            &mut tp as *mut TOKEN_PRIVILEGES,
            0,
            null_mut(),
            null_mut(),
        ) == 0
        {
            let error_code = GetLastError();
            if error_code == 1300 {
                eprintln!("ERROR_NOT_ALL_ASSIGNED (1300): The requested privilege could not be assigned. Ensure the program is running as administrator.");
            } else {
                eprintln!("Failed to adjust token privileges, error: {}", error_code);
            }
            CloseHandle(token_handle);
            CloseHandle(process_handle);
            return false;
        }

        CloseHandle(token_handle);
        CloseHandle(process_handle);

        true
    }
}

#[cfg(target_os = "macos")]
pub fn elevate_privileges(process_name: &str) -> bool {
    use libc::{c_int, getpid, kill};
    use std::ffi::CString;
    use std::process::Command;
    use std::ptr;

    fn find_pid_by_name(process_name: &str) -> Option<i32> {
        let output = Command::new("pgrep")
            .arg(process_name)
            .output()
            .expect("Failed to execute pgrep");

        if output.status.success() {
            if let Ok(pid_str) = String::from_utf8(output.stdout) {
                if let Ok(pid) = pid_str.trim().parse::<i32>() {
                    return Some(pid);
                }
            }
        }

        None
    }

    fn elevate_pid_privileges(pid: i32) -> bool {
        // Try using `sudo` to manipulate process privileges.
        let output = Command::new("sudo")
            .arg("kill")
            .arg("-CONT")
            .arg(format!("{}", pid))
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                println!("Privileges elevated for process with PID: {}", pid);
                return true;
            } else {
                eprintln!(
                    "Failed to elevate privileges, error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return false;
            }
        }
        eprintln!("Failed to run sudo command.");
        false
    }

    if let Some(pid) = find_pid_by_name(process_name) {
        if elevate_pid_privileges(pid) {
            println!(
                "Privileges elevated successfully for process: {}",
                process_name
            );
            return true;
        } else {
            eprintln!("Failed to elevate privileges for process: {}", process_name);
            return false;
        }
    } else {
        eprintln!("Process not found: {}", process_name);
        return false;
    }
}

#[cfg(target_os = "linux")]
pub fn elevate_privileges(process_name: &str) -> bool {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::process::Command;

    // Find the PID by process name
    fn find_pid_by_name(process_name: &str) -> Option<i32> {
        let proc_dir = Path::new("/proc");
        if let Ok(entries) = proc_dir.read_dir() {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    let status_file = entry_path.join("status");
                    if let Ok(file) = File::open(status_file) {
                        for line in io::BufReader::new(file).lines().flatten() {
                            if line.starts_with("Name:") {
                                let proc_name = line.split_whitespace().nth(1).unwrap_or("");
                                if proc_name == process_name {
                                    let pid_str = entry_path.file_name().unwrap().to_str().unwrap();
                                    if let Ok(pid) = pid_str.parse::<i32>() {
                                        return Some(pid);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn elevate_pid_privileges(pid: i32) -> bool {
        // Use `capsh` to elevate privileges using capabilities
        let output = Command::new("capsh")
            .arg("--caps=cap_sys_ptrace+ep")
            .arg("--")
            .arg("sh")
            .arg("-c")
            .arg(format!("kill -0 {}", pid)) // check if the process is accessible
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                return true;
            } else {
                eprintln!(
                    "Failed to elevate privileges, error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return false;
            }
        }
        eprintln!("Failed to run capsh command.");
        false
    }

    if let Some(pid) = find_pid_by_name(process_name) {
        if elevate_pid_privileges(pid) {
            println!(
                "Privileges elevated successfully for process: {}",
                process_name
            );
            return true;
        } else {
            eprintln!("Failed to elevate privileges for process: {}", process_name);
            return false;
        }
    } else {
        eprintln!("Process not found: {}", process_name);
        return false;
    }
}

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
