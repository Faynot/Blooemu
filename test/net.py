import socket
import json


def send_json_to_socket(json_data, host='127.0.0.1', port=7676):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))

        json_str = json.dumps(json_data)
        request = (
            f"POST / HTTP/1.1\r\n"
            f"Host: {host}:{port}\r\n"
            f"Content-Type: application/json\r\n"
            f"Content-Length: {len(json_str)}\r\n"
            f"Connection: close\r\n\r\n"
            f"{json_str}"
        )

        s.sendall(request.encode())

        # Читаем ответ
        response = b""
        while True:
            data = s.recv(1024)
            if not data:
                break
            response += data

        print("Received response from server:")
        print(response.decode())


if __name__ == "__main__":
    json_data = {
        "name": "Alice",
        "age": 30,
        "message": "Hello, server!"
    }

    send_json_to_socket(json_data)
