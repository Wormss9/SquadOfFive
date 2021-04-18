import socket
from _thread import *
import sys


class Server:
    def __init__(self, serverName="GameServer", port=5910,connects=4):
        self.serverName = serverName
        self.port = port
        self.connects=connects
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self.s.bind((self.serverName, self.port))
        except socket.error as e:
            str(e)
        server.s.listen(self.connects)

    def threaded_client(self):
        while True:
            try:
                data = conn.recv(1024).decode("utf-8")
                reply = data.decode("utf-8")

                if not data:
                    print("Disconnected")
                    break
                else:
                    print("Received: ", reply)
                conn.sendall(str.encode(reply))
            except:
                break


server = Server()

while True:
    server.s.listen(4)
    conn, addr = server.s.accept()
    print("Connect to:", addr)

    start_new_thread(server.threaded_client, (conn,))
