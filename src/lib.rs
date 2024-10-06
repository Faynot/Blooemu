pub mod macros;
mod network;
pub mod popups;
mod process;
pub mod utils;
pub mod mouse_position;

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
    open
};

pub use mouse_position::Mouse;


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

    //#[test]
    //fn test_close_socket() {
    //    let addr = "127.0.0.1:8080".parse().unwrap();
    //    match close_socket(addr) {
    //        Ok(_) => println!("Сокет закрыт"),
    //        Err(e) => println!("Ошибка: {}", e),
    //    }
    //}

    // #[test]
    // fn test_send_data() {
    //     let address = "127.0.0.1:8080";
    //     let request = "GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    //
    //     match send_data(address, request) {
    //         Ok(response) => println!("Response:\n{}", response),
    //         Err(e) => eprintln!("Error: {}", e),
    //     }
    // }

    //#[test]
    //fn test_resolve_hostname() {
    //    let hostname = "example.com";
    //    match resolve_hostname(hostname) {
    //        Ok(ip) => println!("IP address of {} is {}", hostname, ip),
    //        Err(e) => println!("Error: {}", e),
    //    }
    //}

    //use std::net::TcpStream;
    //use std::thread;
    //use std::time::Duration;

    //#[test]
    //fn test_listen_socket() {
    //    let addr = "127.0.0.1:8081";
    //
    //    // Running a socket on a separate thread
    //    let handle = thread::spawn(move || {
    //        listen_socket(addr).expect("Failed to start socket");
    //    });
    //
    //    // Wait for some time for the socket to start
    //    thread::sleep(Duration::from_millis(100));
    //
    //    // Attempting to connect to a socket
    //    let result = TcpStream::connect(addr);
    //    assert!(result.is_ok(), "Failed to connect to the socket");
    //
    //    // Socket Shutdown Check
    //    handle.join().unwrap();
    //}

    //use std::thread;
    //use std::net::{TcpListener, SocketAddr};
    //use std::io::Write;
    //use std::io::Read;

    //#[test]
    //fn test_connect_socket() {
    //    let listener_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    //    let listener = TcpListener::bind(listener_addr).expect("Failed to bind listener");
    //
    //    // Spawn a thread to accept connections
    //    thread::spawn(move || {
    //        if let Ok((mut stream, _)) = listener.accept() {
    //            let mut buffer = [0; 1024];
    //            stream.read(&mut buffer).expect("Failed to read from stream");
    //            stream.write_all(b"Hello, client!").expect("Failed to write to stream");
    //        }
    //    });
    //
    //    // Test connection to the server
    //    let mut stream = connect_socket("127.0.0.1:8080").expect("Failed to connect to server");
    //
    //    // Write some data to test the connection
    //    stream.write_all(b"Hello, server!").expect("Failed to send data");
    //
    //    let mut response = String::new();
    //    stream.read_to_string(&mut response).expect("Failed to read response");
    //    assert_eq!(response, "Hello, client!");
    //}

    //use std::thread;
    //
    //#[test]
    //fn test_socket_creation_and_connection() {
    //    // Create a socket in a separate thread
    //    let socket_thread = thread::spawn(|| {
    //        let addr = "0.0.0.0:8080"; // Local Listening Address
    //        listen_socket(addr).expect("Failed to create socket");
    //    });
    //
    //    // Delay to allow time for the socket to start
    //    thread::sleep(std::time::Duration::from_secs(1));
    //
    //    // Connecting to a remote server
    //    let remote_address = "62.217.179.35:80";
    //    match connect_socket(remote_address) {
    //        Ok(stream) => {
    //            println!("Successfully connected to remote server at {}", remote_address);
    //            assert!(stream.peer_addr().is_ok(), "Failed to get peer address");
    //        },
    //        Err(e) => {
    //            panic!("Failed to connect to remote server: {}", e);
    //        }
    //    }
    //
    //    // Waiting for the flow to complete
    //    socket_thread.join().expect("Socket thread panicked");
    //}

    //#[test]
    //fn test_get_interface_name() {
    //    let interface_name = get_interface_name();
    //    assert!(interface_name.is_some(), "No network interface found");
    //    println!("Network interface name: {:?}", interface_name);
    //}

    //#[test]
    //fn test_get_mac_address() {
    //    let mac_address = get_mac_address();
    //    println!("{:?}", mac_address);
    //}

    #[test]
    fn test_get_mouse_position() {
        // Using the get_mouse_position feature from mouse_position
        match Mouse::get_mouse_position() {
            Mouse::Position { x, y } => {
                println!("Mouse Position: x = {}, y = {}", x, y);
            },
            Mouse::Error => {
                eprintln!("Failed to get mouse position");
            }
        }
    }


}
