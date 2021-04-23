import socket
import json
from _thread import *


def print_type(text: str, data):
    """
    Prints text,type(data),data
    """
    print(text, ": ", type(data), " ", str(data))


def print_info(text: str, data):
    print(text, ": ", str(data).replace('\n', ' '))


def bytes_to_dict(received):
    """
    Converts bytes object to dict.

    :param received: bytes
    :return: dict
    """

    def return_error(er):
        print_type("bytes_to_dict", received)
        return to_dict("error", er)

    try:
        translated = json.loads(received.decode())
    except error as e:
        return return_error(e)
    if type(translated) != dict:
        return return_error(400)
    return translated


def dict_to_bytes(to_send):
    """
    Converts dict to bytes

    :param to_send:dict
    :return: bytes
    """
    if type(to_send) != dict:
        print_type("dict_to_bytes", to_send)
        return None
    return json.dumps(to_send).encode('utf-8')


def to_dict(key, value):
    """
    Returns dictionary from key and value

    :param key:
    :param value:
    :return: {key:value}
    """
    return dict({str(key): str(value)})


class NetworkServer:
    """Class responsible for what the server communicates"""

    def __init__(self, server_response_function, port=5910, packet_size=2):
        """
        Initializes a server listening to 5 connections
        
        :param server_response_function:    Response function receiving parameters (dict,socket) 
        :param port:  Server port: int default: 5910
        """""
        # self.socket_connection: socket.socket
        # self.ip_address: str
        self.packet_size = packet_size
        self.server_response_function = server_response_function
        self.listening_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self.listening_socket.bind(("", port))
        except socket.error as e:
            print_type("Port error", e)
        self.listening_socket.listen(5)
        print("Server started.")

    def threaded_client(self, socket_connection, ip_address):
        """Starts connection"""
        print_info("Connected to", ip_address)
        while True:
            try:
                client_response = socket_connection.recv(1024 * self.packet_size)
                if client_response:
                    print_type("Received", client_response)
                    print_type("threaded_client", (bytes_to_dict(client_response), socket_connection.sendall))
                    self.server_response_function(bytes_to_dict(client_response), socket_connection.sendall)
                else:
                    print_info("Disconnected from", ip_address)
                    break
            except:
                break
        self.server_response_function(to_dict("disconnected", ""), socket_connection.sendall)
        socket_connection.close()

    def accept_connection(self):
        """Starts connection as new thread """
        socket_connection, ip_address = self.listening_socket.accept()
        start_new_thread(self.threaded_client, (socket_connection, ip_address,))

    def send(self, response: dict, connection_socket):
        try:
            print_type("ServerNetwork.send", response)
            connection_socket(dict_to_bytes(response))
            print_type("Sent", response)
        except error as e:
            print_type("NetworkClient.send", e)


class NetworkClient:
    """
    Network client responsible for communicating with server.
    """

    def __init__(self, response_function, server_ip, server_port=5910, packet_size=2):
        """
        Initializes Network client
        :param response_function:   Function accepting dictionary as parameter.
        :param server_ip:           Server ip   :str
        :param server_port:         Server port :int default:5910
        :param packet_size:         Size of packet in kilobytes:int default:2
        """
        self.response_function = response_function
        self.server_address = (server_ip, server_port)
        self.connection_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.packet_size = packet_size

    def connect(self):
        try:
            print("Client connecting: ", self.server_address)
            self.connection_socket.connect(self.server_address)
            print("Connected to: ", self.server_address)
            start_new_thread(self.listen, ())
        except error as e:
            print_type("NetworkClient.connect", e)

    def send(self, response: dict):
        try:
            print_type("ClientNetwork.send", response)
            self.connection_socket.sendall(dict_to_bytes(response))
            print_type("Sent", response)
        except error as e:
            print_type("NetworkClient.send", e)

    def listen(self):
        while True:
            try:
                data = self.connection_socket.recv(1024 * 2)
                print_type("Listened to1", data)
                if data:
                    print_type("Listened to2", data)
                    self.response_function(bytes_to_dict(data))
                else:
                    print("Disconnected: ", str(self.server_address))
                    break
            except error as e:
                print_type("NetworkClient.listen", e)
                break
