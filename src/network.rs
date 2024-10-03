use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use serde_json::Value;

pub type RequestHandler = fn(&str, Option<Value>) -> String;

pub fn create_socket(addr: &str, handler: RequestHandler) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let local_addr = listener.local_addr()?;
    println!("Socket running on {}", local_addr);

    let running = Arc::new(AtomicBool::new(true));

    for stream in listener.incoming() {
        let running = Arc::clone(&running);
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr()?;
                println!("Connection from {}", peer_addr);

                let running_clone = Arc::clone(&running);
                let handler_clone = handler;
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
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        println!("Connection closed by the client {}", peer_addr);
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request received from {}: {}", peer_addr, request);

    let (method, path, body) = parse_http_request(&request);

    let json_body: Option<Value> = if method == "POST" && !body.is_empty() {
        serde_json::from_str(&body).ok()
    } else {
        None
    };

    let response_body = handler(&path, json_body);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes())?;
    println!("Response successfully sent to client {}", peer_addr);

    stream.shutdown(std::net::Shutdown::Both)?;
    println!("The connection to the {} client is complete", peer_addr);

    Ok(())
}

fn parse_http_request(request: &str) -> (String, String, String) {
    let lines: Vec<&str> = request.split("\r\n").collect();

    if lines.is_empty() {
        return ("".to_string(), "".to_string(), "".to_string());
    }

    let first_line = lines[0];
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        return ("".to_string(), "".to_string(), "".to_string());
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();

    let mut body = String::new();
    let mut is_body = false;

    for line in lines {
        if line.is_empty() {
            is_body = true;
            continue;
        }
        if is_body {
            body.push_str(line);
        }
    }

    (method, path, body)
}
