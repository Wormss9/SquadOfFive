import socket
import json
from _thread import *

defServer = "25.96.228.106"
defPort = 5910


class Server:
    """Class responsible for what the server communicates"""
    def __init__(self, game, server=defServer, port=defPort):
        """Initializes a server listening to 5 connections"""
        self.conn = ""
        self.address = ""
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.game = game
        try:
            self.s.bind(("", port))
        except socket.error as e:
            str(e)
        self.s.listen(5)
        print("Waiting for connection")

    def threaded_client(self, conn):
        """Starts connection"""
        conn.send(str.encode("Connected"))
        # reply = ""
        while True:
            try:
                data = self.conn.recv(1024 * 2)
                print("Data: ",str(data))
                #reply = self.game.reply(json.loads(data))
                reply="Test reply"
                if not data:
                    print(str(self.address), " disconnected.")
                    break
                else:
                    print("Received: ", reply)
                self.conn.sendall(str.encode(json.dumps(reply)))
            except error:
                print(str(error))
                break
        print("Connection lost.")
        self.conn.close()

    def accept(self):
        """Starts connection as new thread """
        self.conn, self.address = self.s.accept()
        print("Connected to :", self.address)
        start_new_thread(self.threaded_client, (self.conn,))
        # self.threaded_client()


class Client:
    """Class responsible for what the client communicates"""
    def __init__(self, server=defServer, port=defPort):
        self.client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.port = port
        self.server = server
        self.address = (self.server, self.port)
        self.pos = self.connect()

    def connect(self):
        try:
            self.client.connect(self.address)
            return self.client.recv(1024 * 2).decode()
        except error as e:
            str(e)

    def send(self, data: dict):
        if data is str:
            data = {"info", data}
        try:
            self.client.send(str.encode(json.dumps(data)))
            return self.client.recv(1024 * 2)
        except error as e:
            print(e)
