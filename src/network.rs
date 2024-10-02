use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub type RequestHandler = fn(&str) -> String;

pub fn create_socket(addr: &str, handler: RequestHandler) -> io::Result<()> {
    // Создаем сокет по указанному адресу
    let listener = TcpListener::bind(addr)?;
    let local_addr = listener.local_addr()?;
    println!("Socket running on {}", local_addr);

    // Флаг для управления состоянием сервера
    let running = Arc::new(AtomicBool::new(true));

    for stream in listener.incoming() {
        let running = Arc::clone(&running);
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr()?;
                println!("Connection from {}", peer_addr);

                // Обработка подключения в отдельном потоке
                let running_clone = Arc::clone(&running);
                let handler_clone = handler; // Клонируем обработчик
                thread::spawn(move || {
                    if let Err(e) = handle_client(&mut stream, &running_clone, peer_addr, handler_clone) {
                        eprintln!("Client processing error {}: {}", peer_addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }

        // Сервер продолжает слушать новые соединения
        if !running.load(Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream, _running: &Arc<AtomicBool>, peer_addr: std::net::SocketAddr, handler: RequestHandler) -> io::Result<()> {
    let mut buffer = [0; 512];

    // Чтение данных
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        println!("Connection closed by the client {}", peer_addr);
        return Ok(());
    }

    // Отображение полученных данных
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Request received from {}: {}", peer_addr, request);

    // Обработка запроса с помощью пользовательского обработчика
    let response_body = handler(&request);
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body);

    // Отправка ответа
    stream.write_all(response.as_bytes())?;
    println!("Response successfully sent to client {}", peer_addr);

    // Завершение соединения после отправки
    stream.shutdown(std::net::Shutdown::Both)?;
    println!("The connection to the {} client is complete", peer_addr);

    Ok(())
}
