import socket

SERVER = ('127.0.0.1', 6379)

def send_response_template(client_socket, message, expected_response):
    client_socket.sendall(message)
    response = client_socket.recv(1024)
    assert response == expected_response

def test_ping():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        send_response_template(client_socket, b'PING\r\n', b'+PONG\r\n')

def test_ten_pings():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for _ in range(10):
            message = b'PING\r\n'
            send_response_template(client_socket, message, b'+PONG\r\n')

def test_echo():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for _ in range(10):
            message = b'ECHO value\r\n'
            send_response_template(client_socket, message, b'$5\r\nvalue\r\n')

def test_multiple_echoes():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for i in range(20):
            message = f'ECHO value{i}\r\n'.encode()
            expected_response = f'${5+len(str(i))}\r\nvalue{i}\r\n'.encode()
            send_response_template(client_socket, message, expected_response)
