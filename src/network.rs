use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub type RequestHandler = fn(&str) -> String;

pub fn create_socket(addr: &str, handler: RequestHandler) -> io::Result<()> {
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

                // Handling a connection in a separate thread
                let running_clone = Arc::clone(&running);
                let handler_clone = handler; // Clone the handler
                thread::spawn(move || {
                    if let Err(e) =
                        handle_client(&mut stream, &running_clone, peer_addr, handler_clone)
                    {
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

fn handle_client(
    stream: &mut TcpStream,
    _running: &Arc<AtomicBool>,
    peer_addr: std::net::SocketAddr,
    handler: RequestHandler,
) -> io::Result<()> {
    let mut buffer = [0; 512];

    // Reading data
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        println!("Connection closed by the client {}", peer_addr);
        return Ok(());
    }

    // Displaying the received data
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request received from {}: {}", peer_addr, request);

    // Handling a Request with a Custom Handler
    let response_body = handler(&request);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    // Send a response
    stream.write_all(response.as_bytes())?;
    println!("Response successfully sent to client {}", peer_addr);

    // Termination of the connection after sending
    stream.shutdown(std::net::Shutdown::Both)?;
    println!("The connection to the {} client is complete", peer_addr);

    Ok(())
}