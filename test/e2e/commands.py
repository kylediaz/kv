import socket

SERVER = ('127.0.0.1', 6379)

def send_response_template(client_socket, message, expected_response):
    client_socket.sendall(message)
    response = client_socket.recv(1024)
    assert response == expected_response

def test_ping():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        send_response_template(client_socket, b'*1\r\n$4\r\nPING\r\n', b'+PONG\r\n')

def test_ten_pings():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for _ in range(10):
            message = b'*1\r\n$4\r\nPING\r\n'
            send_response_template(client_socket, message, b'+PONG\r\n')

def test_echo():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for _ in range(10):
            message = b'*2\r\n$4\r\nECHO\r\n$5\r\nvalue\r\n'
            send_response_template(client_socket, message, b'+value\r\n')

def test_multiple_echoes():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for i in range(20):
            message = f'*2\r\n$4\r\nECHO\r\n${5 + len(str(i))}\r\nvalue{i}\r\n'.encode()
            expected_response = f'+value{i}\r\n'.encode()
            send_response_template(client_socket, message, expected_response)

def test_get_nil():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        message = b'*2\r\n$3\r\nGET\r\n$8\r\nfake-key\r\n'
        send_response_template(client_socket, message, b'$-1\r\n')

def test_multiple_get_nil():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for _ in range(20):
            message = b'*2\r\n$3\r\nGET\r\n$8\r\nfake-key\r\n'
            send_response_template(client_socket, message, b'$-1\r\n')

def test_set():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        message = b'*3\r\n$3\r\nSET\r\n$12\r\ntest-set-key\r\n$5\r\nvalue\r\n'
        send_response_template(client_socket, message, b'+OK\r\n')

def test_set_and_get():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        message = b'*3\r\n$3\r\nSET\r\n$20\r\ntest-set-and-get-key\r\n$5\r\nvalue\r\n'
        send_response_template(client_socket, message, b'+OK\r\n')
        message = b'*2\r\n$3\r\nGET\r\n$20\r\ntest-set-and-get-key\r\n'
        send_response_template(client_socket, message, b'$5\r\nvalue\r\n')

def test_multiple_set_and_get():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect(SERVER)
        for i in range(10):
            message = f'*3\r\n$3\r\nSET\r\n$22\r\ntest-set-and-get-key-{i}\r\n$6\r\nvalue{i}\r\n'.encode()
            send_response_template(client_socket, message, b'+OK\r\n')
        for i in range(10):
            message = f'*2\r\n$3\r\nGET\r\n$22\r\ntest-set-and-get-key-{i}\r\n'.encode()
            expected_response = f'$6\r\nvalue{i}\r\n'.encode()
            send_response_template(client_socket, message, expected_response)
