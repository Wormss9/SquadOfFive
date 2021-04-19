import socket
import json
from _thread import *

server = "25.96.228.106"
port = 5910


class Server:
    def __init__(self, game, server=server, port=port):
        self.conn = ""
        self.addr = ""
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.game = game
        try:
            self.s.bind((server, port))
        except socket.error as e:
            str(e)
        self.s.listen(5)
        print("Waiting for connection")

    def threaded_client(self, conn):
        conn.send(str.encode("Connected"))
        reply = ""
        while True:
            try:
                data = self.conn.recv(1024 * 2)
                reply = json.dump(self.game.reply(json.loads(data)))
                if not data:
                    print(str(self.addr), " disconected.")
                    break
                else:
                    print("Received: ", reply)
                self.conn.sendall(str.encode(reply))
            except:
                break
        print("Connection lost.")
        self.conn.close()

    def accept(self):
        self.conn, self.addr = self.s.accept()
        print("Connected to :", self.addr)
        start_new_thread(self.threaded_client, (self.conn,))
        # self.threaded_client()


class Client:
    def __init__(self, server=server, port=port):
        self.client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.port = port
        self.server = server
        self.addr = (self.server, self.port)
        self.pos = self.connect()

    def getPos(self):
        return self.pos

    def connect(self):
        try:
            self.client.connect(self.addr)
            return self.client.recv(1024 * 2).decode()
        except error as e:
            str(e)

    def send(self, data):
        try:
            self.client.send(str.encode(data))
            return self.client.recv(1024 * 2).decode()
        except socket.error as e:
            print(e)
