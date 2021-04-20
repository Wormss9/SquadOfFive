import socket
import json
from _thread import *

defServer = "25.96.228.106"
defPort = 5910


def print_type(text: str, data):
    print(text, " ", type(data), " ", data)


def bytes_to_json(received):
    try:
        received = json.loads(received.decode())
    except:
        print_type("Bad translation", received)
        return {"error": 400}
    if type(received) == dict:
        return received
    else:
        print_type("Bad translation", received)
        return {"error": 400}


def json_to_bytes(to_send):
    return json.dumps(to_send).encode('utf-8')


class Server:
    """Class responsible for what the server communicates"""

    def __init__(self, game_server_logic, port=defPort):
        """Initializes a server listening to 5 connections"""
        self.conn = ""
        self.address = ""
        self.s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.game_server_logic = game_server_logic
        try:
            self.s.bind(("", port))
        except socket.error as e:
            str(e)
        self.s.listen(5)
        print("Waiting for connection")

    def threaded_client(self, conn):
        """Starts connection"""
        conn.sendall(json_to_bytes({"connected": True}))
        print(str(self.address), " connected.")
        while True:
            try:
                data = self.conn.recv(1024 * 2)
                print_type("Received: ", data)
                # Sends json to reply
                reply = self.game_server_logic.reply(bytes_to_json(data), conn)
                if not data:
                    print(str(self.address), " disconnected.")
                    break
                else:
                    print("Sent: '", reply, "' To:", self.address[1])
                # Send reply to client
                self.conn.sendall(json_to_bytes(reply))
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
        self.gameClient = ""

    def connect(self):
        try:
            print("Client connecting: ",self.address)
            self.client.connect(self.address)
            answer = bytes_to_json(self.client.recv(1024 * 2))
            for key in answer:
                self.gameClient.answer(key, answer[key], self.client)
            print("Connected to: ")
        except error as e:
            str(e)

    def send(self, data: dict):
        if data is str:
            data = {"info", data}
        try:
            self.client.sendall(json_to_bytes(data))
            answer = bytes_to_json(self.client.recv(1024 * 2))
            # todo
            print_type("Answer:",answer)
            for key in answer:
                self.gameClient.answer(key, answer[key], self.client)
        except error as e:
            print(e)
