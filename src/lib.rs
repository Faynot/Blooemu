pub mod macros;
mod network;
pub mod popups;
mod process;
pub mod utils;
pub mod mouse_position;
mod file_system;

pub use popups::
{
    alert_message,
    error_message
};


pub use network::
{
    close_socket,
    connect_socket,
    create_socket,
    get_external_ip,
    get_hostname,
    get_interface_name,
    get_local_ip,
    get_network_interfaces,
    is_network_available,
    listen_socket,
    resolve_hostname,
    send_data,
    get_mac_address,

};


pub use process::
{
    elevate_privileges,
    get_all_processes,
    get_pid,
    get_process_cpu_usage,
    get_process_memory_usage,
    get_process_name,
};


pub use utils::
{
    close,
    open,
    set_timer,
    cancel_timer
};

pub use file_system::{
    create_file,
    open_file,
    read_file,
    write_file,
    create_directory,
    delete_directory,
    WriteMode,
    move_directory,
    get_directory_contents,
    has_file_access,
    has_directory_access,
    get_file_creation_date,
    get_file_modification_date,
    get_file_size,
    get_file_owner,
    create_symlink
};

pub use mouse_position::Mouse;


#[cfg(test)]
mod tests {
    // use std::io;
    use super::*;

    #[test]
    fn test_error_message() {
        let message = "Test error message";
        error_message(message, "Test Error Title", None);
    }
//
//    #[test]
//    fn test_open() {
//        let path = "C:/Users/Happy PC/Desktop/test.txt";
//        open(path);
//    }
//
//    #[test]
//    fn test_get_pid() {
//        let task_name = "notepad";
//        if let Some(pid) = get_pid(task_name) {
//            println!("Found task {} with PID: {}", task_name, pid);
//        } else {
//            eprintln!("Task {} not found", task_name);
//        }
//    }
//
//    #[test]
//    fn test_alert_macro() {
//        alert!(
//            "Test with yes and no callbacks",
//            "Custom Title",
//            "yesno",
//            || {
//                error!("you choose yes");
//            },
//            || {
//                error!("you choose no");
//            }
//        );
//    }
//
//    #[test]
//    fn test_error_macro() {
//        error!("Test error macro");
//        error!("Test error with title", "Critical Error");
//        error!("Test error with callback", "Critical Error", || {
//            println!("Error callback executed");
//        });
//    }
//
//    #[test]
//    fn test_close() {
//        if let Some(pid) = get_pid("notepad.exe") {
//            println!("Found process ID: {}", pid);
//            let namepid = get_process_name(pid);
//            println!("{:?}", namepid);
//        } else {
//            println!("Process not found.");
//        }
//
//        #[test]
//        fn test_get_memory_use() {
//            match get_process_memory_usage("notepad.exe") {
//                Some(memory_usage) => println!("{}", memory_usage),
//                None => println!("Process not found"),
//            }
//        }
//
//        #[test]
//        fn test_get_cpu_use() {
//            match get_process_cpu_usage("notepad.exe") {
//                Some(cpu_usage) => println!("{}", cpu_usage),
//                None => println!("Process not found"),
//            }
//        }
//    }
//
//    #[test]
//    fn test_get_all_processes() {
//        let all = get_all_processes();
//        println!("{:?}", all);
//    }
//
//    #[test]
//    fn test_elevate_privileges() {
//        let process_name = "notepad.exe"; // Replace with an actual running process name
//        let result = elevate_privileges(process_name);
//
//        assert!(
//            result,
//            "Failed to elevate privileges for process: {}",
//            process_name
//        );
//    }
//
//    #[test]
//    fn test_close_socket() {
//        let addr = "127.0.0.1:8080".parse().unwrap();
//        match close_socket(addr) {
//            Ok(_) => println!("Сокет закрыт"),
//            Err(e) => println!("Ошибка: {}", e),
//        }
//    }
//
//    #[test]
//    fn test_send_data() {
//        let address = "127.0.0.1:8080";
//        let request = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
//
//        match send_data(address, request) {
//            Ok(response) => println!("Response:\n{}", response),
//            Err(e) => eprintln!("Error: {}", e),
//        }
//    }
//
//    #[test]
//    fn test_resolve_hostname() {
//        let hostname = "example.com";
//        match resolve_hostname(hostname) {
//            Ok(ip) => println!("IP address of {} is {}", hostname, ip),
//            Err(e) => println!("Error: {}", e),
//        }
//    }
//
//    use std::net::TcpStream;
//    use std::thread;
//    use std::time::Duration;
//
//    #[test]
//    fn test_listen_socket() {
//        let addr = "127.0.0.1:8081";
//
//        // Running a socket on a separate thread
//        let handle = thread::spawn(move || {
//            listen_socket(addr).expect("Failed to start socket");
//        });
//
//        // Wait for some time for the socket to start
//        thread::sleep(Duration::from_millis(100));
//
//        // Attempting to connect to a socket
//        let result = TcpStream::connect(addr);
//        assert!(result.is_ok(), "Failed to connect to the socket");
//
//        // Socket Shutdown Check
//        handle.join().unwrap();
//    }
//
//    use std::thread;
//    use std::net::{TcpListener, SocketAddr};
//    use std::io::Write;
//    use std::io::Read;
//
//    #[test]
//    fn test_connect_socket() {
//        let listener_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//        let listener = TcpListener::bind(listener_addr).expect("Failed to bind listener");
//
//        // Spawn a thread to accept connections
//        thread::spawn(move || {
//            if let Ok((mut stream, _)) = listener.accept() {
//                let mut buffer = [0; 1024];
//                stream.read(&mut buffer).expect("Failed to read from stream");
//                stream.write_all(b"Hello, client!").expect("Failed to write to stream");
//            }
//        });
//
//        // Test connection to the server
//        let mut stream = connect_socket("127.0.0.1:8080").expect("Failed to connect to server");
//
//        // Write some data to test the connection
//        stream.write_all(b"Hello, server!").expect("Failed to send data");
//
//        let mut response = String::new();
//        stream.read_to_string(&mut response).expect("Failed to read response");
//        assert_eq!(response, "Hello, client!");
//    }
//
//    use std::thread;
//
//    #[test]
//    fn test_socket_creation_and_connection() {
//        // Create a socket in a separate thread
//        let socket_thread = thread::spawn(|| {
//            let addr = "0.0.0.0:8080"; // Local Listening Address
//            listen_socket(addr).expect("Failed to create socket");
//        });
//
//        // Delay to allow time for the socket to start
//        thread::sleep(Duration::from_secs(1));
//
//        // Connecting to a remote server
//        let remote_address = "62.217.179.35:80";
//        match connect_socket(remote_address) {
//            Ok(stream) => {
//                println!("Successfully connected to remote server at {}", remote_address);
//                assert!(stream.peer_addr().is_ok(), "Failed to get peer address");
//            },
//            Err(e) => {
//                panic!("Failed to connect to remote server: {}", e);
//            }
//        }
//
//        // Waiting for the flow to complete
//        socket_thread.join().expect("Socket thread panicked");
//    }
//
//    #[test]
//    fn test_get_interface_name() {
//        let interface_name = get_interface_name();
//        assert!(interface_name.is_some(), "No network interface found");
//        println!("Network interface name: {:?}", interface_name);
//    }
//
//    #[test]
//    fn test_get_mac_address() {
//        let mac_address = get_mac_address();
//        println!("{:?}", mac_address);
//    }
//
//    #[test]
//    fn test_get_mouse_position() {
//        // Using the get_mouse_position feature from mouse_position
//        match Mouse::get_mouse_position() {
//            Mouse::Position { x, y } => {
//                println!("Mouse Position: x = {}, y = {}", x, y);
//            },
//            Mouse::Error => {
//                eprintln!("Failed to get mouse position");
//            }
//                    }
//            }
//
//    // Imports to delete files after the test
//    use std::fs;
//    use std::path::Path;
//
//    #[test]
//    fn test_move_directory() {
//        let source = "test_dir";
//        let destination = "moved_dir";
//
//        create_directory(source).unwrap();
//        move_directory(source, destination).unwrap();
//
//        assert!(!Path::new(source).exists());
//        assert!(Path::new(destination).exists());
//
//        delete_directory(destination).unwrap();
//    }
//
//    #[test]
//    fn test_get_directory_contents() {
//        let dir = "content_dir";
//        create_directory(dir).unwrap();
//
//        let file_path = format!("{}/test_file.txt", dir);
//        create_file(&file_path).unwrap();
//
//        let contents = get_directory_contents(dir).unwrap();
//        assert_eq!(contents.len(), 1);
//        assert_eq!(contents[0], Path::new(&file_path));
//
//        delete_directory(dir).unwrap();
//    }
//
//
//    #[test]
//    fn test_create_file() {
//        let path = "test_create_file.txt";
//        assert!(create_file(path).is_ok());
//        assert!(Path::new(path).exists());
//        //fs::remove_file(path).unwrap();
//    }

//    #[test]
//    fn test_open_file() {
//        let path = "test_create_file.txt";
//        create_file(path).unwrap();
//        assert!(open_file(path).is_ok());
//        fs::remove_file(path).unwrap();
//    }
//
//    #[test]
//    fn test_read_file() {
//        let path = "test_create_file.txt";
//        let content = "Hello, world!";
//        write_file(path, content, WriteMode::Overwrite).unwrap();
//        let read_content = read_file(path).unwrap();
//        assert_eq!(content, read_content);
//        fs::remove_file(path).unwrap();
//    }

//    #[test]
//    fn test_write_file() {
//        let path = "test_write_file.txt";
//        let content = "Hello, Rust!";
//        write_file(path, content, WriteMode::Overwrite).unwrap();
//        let read_content = read_file(path).unwrap();
//        assert_eq!(content, read_content);
//
//        let append_content = " Append this!";
//        write_file(path, append_content, WriteMode::Append).unwrap();
//        let read_content = read_file(path).unwrap();
//        assert_eq!(read_content, format!("{}{}", content, append_content));
//
//        let append_content_again = " Another append!";
//        write_file(path, append_content_again, WriteMode::Append).unwrap();
//        let read_content = read_file(path).unwrap();
//        assert_eq!(read_content, format!("{}{}{}", content, append_content, append_content_again));
//
//        // Удаляем "Rust!" и проверяем, что пробелы корректно обработаны
//        write_file(path, "Rust!", WriteMode::Delete("Rust!".to_string())).unwrap();
//        let read_content = read_file(path).unwrap();
//        assert_eq!(read_content, "Hello, Append this! Another append!");  // Проверка
//
//        fs::remove_file(path).unwrap();
//    }

//    #[test]
//    fn test_create_directory() {
//        let dir = "test_directory";
//        assert!(create_directory(dir).is_ok());
//        assert!(Path::new(dir).exists());
//        fs::remove_dir_all(dir).unwrap();
//    }

//    #[test]
//    fn test_delete_directory() {
//        let dir = "x";
//        create_directory(dir).unwrap();
//        assert!(delete_directory(dir).is_ok());
//        assert!(!Path::new(dir).exists());
//    }

//    #[test]
//    fn test_has_file_access() {
//        let path = "test_create_file.txt";
//        if has_file_access(path) {
//            println!("У вас есть доступ к файлу: {}", path);
//        } else {
//            println!("Нет доступа к файлу: {}", path);
//        }
//    }

//    #[test]
//    fn test_get_file_size() -> Result<(), Box<dyn std::error::Error>> {
//        let path = "test_create_file.txt";
//        let size = get_file_size(path)?;
//        println!("{}", size);
//
//        Ok(())
//    }

//    #[test]
//    fn test_file_create_date() -> Result<(), Box<dyn std::error::Error>> {
//        let path = "test_create_file.txt";
//        let creation_date = get_file_creation_date(path)?;
//        println!("{:?}", creation_date);
//
//        Ok(())
//    }

//    #[test]
//    fn test_file_morification() -> Result<(), Box<dyn std::error::Error>> {
//        let path = "test_create_file.txt";
//        let modification_date = get_file_modification_date(path)?;
//        println!("{:?}", modification_date);
//
//        Ok(())
//    }



//    #[test]
//    fn test_get_file_owner() -> io::Result<()> {
//        let path = Path::new("path/to/your/file");
//        match get_file_owner(path) {
//            Ok(owner) => println!("File owner: {:?}", owner),
//            Err(e) => eprintln!("Failed to get file owner: {}", e),
//        }
//        Ok(())
//    }

    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_set_timer() {
        let seconds = 5;

        let tx = set_timer(seconds, || {
            println!("Таймер завершил работу!");
        });

        println!("Программа ожидает завершения таймера...");
        thread::sleep(Duration::from_secs(2)); // Ожидаем 2 секунды

        cancel_timer(tx); // Отменяем таймер

        println!("Таймер отменен!");
        thread::sleep(Duration::from_secs(4));
    }


}
