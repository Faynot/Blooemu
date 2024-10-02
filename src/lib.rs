pub mod macros;
pub mod popups;
mod process;
pub mod utils;
mod network;

pub use popups::{
    alert_message,
    error_message
};

pub use process::{
    get_pid,
    get_process_cpu_usage,
    get_process_memory_usage,
    get_process_name,
    get_all_processes,
    elevate_privileges,
};
pub use utils::{
    close,
    open
};
pub use network::create_socket;


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test_error_message() {
    //    let message = "Test error message";
    //    error_message(message, "Test Error Title", None);
    //}
//
    //#[test]
    //fn test_open() {
    //    let path = "C:/Users/Happy PC/Desktop/test.txt";
    //    open(path);
    //}
//
    //#[test]
    //fn test_get_pid() {
    //    let task_name = "notepad";
    //    if let Some(pid) = get_pid(task_name) {
    //        println!("Found task {} with PID: {}", task_name, pid);
    //    } else {
    //        eprintln!("Task {} not found", task_name);
    //    }
    //}
//
    //#[test]
    //fn test_alert_macro() {
    //    alert!(
    //        "Test with yes and no callbacks",
    //        "Custom Title",
    //        "yesno",
    //        || {
    //            error!("you choose yes");
    //        },
    //        || {
    //            error!("you choose no");
    //        }
    //    );
    //}
//
    //#[test]
    //fn test_error_macro() {
    //    error!("Test error macro");
    //    error!("Test error with title", "Critical Error");
    //    error!("Test error with callback", "Critical Error", || {
    //        println!("Error callback executed");
    //    });
    //}
//
    //#[test]
    //fn test_close() {
    //    if let Some(pid) = get_pid("notepad.exe") {
    //        println!("Found process ID: {}", pid);
    //        let namepid = get_process_name(pid);
    //        println!("{:?}", namepid);
    //    } else {
    //        println!("Process not found.");
    //    }
//
    //    #[test]
    //    fn test_get_memory_use() {
    //        match get_process_memory_usage("notepad.exe") {
    //            Some(memory_usage) => println!("{}", memory_usage),
    //            None => println!("Process not found"),
    //        }
    //    }
//
    //    #[test]
    //    fn test_get_cpu_use() {
    //        match get_process_cpu_usage("notepad.exe") {
    //            Some(cpu_usage) => println!("{}", cpu_usage),
    //            None => println!("Process not found"),
    //        }
    //    }
    //}
//
    //#[test]
    //fn test_get_all_processes() {
    //    let all = get_all_processes();
    //    println!("{:?}", all);
    //}
//
    //#[test]
    //fn test_elevate_privileges() {
    //    let process_name = "notepad.exe"; // Replace with an actual running process name
    //    let result = elevate_privileges(process_name);
//
    //    assert!(
    //        result,
    //        "Failed to elevate privileges for process: {}",
    //        process_name
    //    );
    //}

    use std::net::TcpStream;
    use std::thread;

    #[test]
    fn test_create_socket() {
        // Локальный адрес для теста
        let addr = "localhost:8080";

        // Запуск сокета в отдельном потоке
        let server_thread = thread::spawn(move || {
            create_socket(addr).expect("Не удалось создать сокет");
        });

        // Даем серверу время для инициализации
        thread::sleep(std::time::Duration::from_millis(100));

        // Попытка подключения к сокету
        match TcpStream::connect(addr) {
            Ok(_) => {
                println!("Тестовое подключение успешно");
            }
            Err(e) => {
                panic!("Не удалось подключиться к сокету: {}", e);
            }
        }

        // Завершаем тест
        server_thread.join().expect("Ошибка завершения сервера");
    }


}
