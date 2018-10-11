import socket


def client(host, port, data):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((host, port))
    s.sendall(data)
    data = s.recv(1024)
    s.close()
    print('Received', data.decode())


if __name__ == "__main__":
    host = "127.0.0.1"
    port = 5011
    data = b'Hello, world'