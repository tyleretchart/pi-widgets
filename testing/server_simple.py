import socket


def server(host, port):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((host, port))
    while True:
        s.listen(1)
        conn, addr = s.accept()
        print('Connected by', addr)
        while True:
            try:
                data = conn.recv(1024)
                if not data:
                    break

                print(type(repr(data)))
                print("Client Says: " + repr(data))

                conn.sendall(b"Server Says: hi")

            except socket.error:
                print("Error Occured.")
                break

        conn.close()
        print()


if __name__ == "__main__":
    host = '127.0.0.1'  # Symbolic name meaning all available interfaces
    port = 5004  # Arbitrary non-privileged port
    server(host, port)