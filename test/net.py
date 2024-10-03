import socket


def send_request(host, port, message):
    # Создаем сокет
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # Подключаемся к серверу
        s.connect((host, port))

        # Отправляем запрос
        s.sendall(message.encode('utf-8'))

        # Получаем ответ
        response = s.recv(4096).decode('utf-8')

    return response


if __name__ == "__main__":
    host = '127.0.0.1'  # Адрес сервера
    port = 8080  # Порт сервера, к которому вы изменили bind_socket

    # Пример GET-запроса
    request_message = "GET / HTTP/1.1\r\nHost: {}\r\n\r\n".format(host)

    # Отправляем запрос и получаем ответ
    response = send_request(host, port, request_message)

    # Выводим ответ на экран
    print("Response from server:")
    print(response)
