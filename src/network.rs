use serde_json::Value;
use std::io::{self, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub type RequestHandler = fn(&str, Option<Value>) -> String;

pub fn get_external_ip() -> io::Result<String> {
    // The URL of the service that returns the external IP address
    let address = "api.ipify.org:80";
    let request = "GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n";

    // Connecting to the server
    let stream_result = TcpStream::connect(address.to_socket_addrs()?.next().unwrap());

    let mut stream = match stream_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Connection error: {}", e);
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No internet connection",
            ));
        }
    };

    // Sending an HTTP request
    if let Err(e) = stream.write_all(request.as_bytes()) {
        eprintln!("Error sending request: {}", e);
        return Err(e);
    }

    // Reading an answer
    let mut response = Vec::new();
    if let Err(e) = stream.read_to_end(&mut response) {
        eprintln!("Error when reading the answer: {}", e);
        return Err(e);
    }

    // Convert an answer to a string
    let response_str = String::from_utf8_lossy(&response);

    // Parsing an IP address from a response
    if let Some(ip_start) = response_str.find("\r\n\r\n") {
        let ip_address = &response_str[ip_start + 4..]; // Removing headings
        return Ok(ip_address.trim().to_string());
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to parse IP address",
    ))
}

pub fn get_local_ip() -> io::Result<IpAddr> {
    let listener = TcpListener::bind("0.0.0.0:0")?;
    let local_addr = listener.local_addr()?;
    Ok(local_addr.ip())
}
pub fn is_network_available(addr: &str) -> bool {
    // Add the standard port 80 if it is not specified
    let socket_addr = if addr.contains(':') {
        match addr.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("Address parsing error: {}", e);
                return false;
            }
        }
    } else {
        format!("{}:80", addr) // Adding Port 80
            .parse::<SocketAddr>()
            .expect("Address parsing error")
    };

    // Timed Out Connection Attempt
    TcpStream::connect_timeout(&socket_addr, Duration::from_secs(2)).is_ok()
}

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
    peer_addr: SocketAddr,
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
