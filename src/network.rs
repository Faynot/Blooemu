use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn create_socket(addr: &str) -> io::Result<()> {
    // Create a socket at the specified address
    let listener = TcpListener::bind(addr)?;
    let local_addr = listener.local_addr()?;
    println!("Socket running on {}", local_addr);

    // Flag for server state management
    let running = Arc::new(AtomicBool::new(true));

    for stream in listener.incoming() {
        let running = Arc::clone(&running);
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr()?;
                println!("Connection from {}", peer_addr);

                // Processing the connection in a separate thread
                let running_clone = Arc::clone(&running);
                thread::spawn(move || {
                    if let Err(e) = handle_client(&mut stream, &running_clone, peer_addr) {
                        eprintln!("Client processing error {}: {}", peer_addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }

        // The server continues to listen for new connections
        if !running.load(Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream, _running: &Arc<AtomicBool>, peer_addr: std::net::SocketAddr) -> io::Result<()> {
    let mut buffer = [0; 512];

    // Reading data
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        println!("Connection closed by the client {}", peer_addr);
        return Ok(());
    }

    // Displaying the obtained data
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request received from {}: {}", peer_addr, request);

    // Sending a response
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello Blooemu!";
    stream.write_all(response)?;
    println!("Response successfully sent to client {}", peer_addr);

    // Complete the connection after sending
    stream.shutdown(std::net::Shutdown::Both)?;
    println!("The connection to the {} client is complete", peer_addr);

    Ok(())
}
