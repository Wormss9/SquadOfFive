import socket
import json
from _thread import start_new_thread, error


def print_type(text: str, data):
    """
    Prints text,type(data),data
    """
    print(text, ": ", type(data), " ", str(data))


def print_info(text: str, data):
    print(text, ": ", str(data).replace('\n', ' '))


def bytes_to_dict(received: bytes):
    """
    Converts bytes object to dict.

    :param received: bytes
    :return: dict
    """
    to_handle = (str(received)[3:-2].split("}{"))
    translateds = []
    for block in to_handle:
        block = '{' + block + '}'
        x = 50
        y = -5
        if len(block) > x:
            print("translatig: ", str(block)[:x + y], str(block)[y:])
        else:
            print("translatig: ", str(block))
        try:
            translateds.append(json.loads(block))
        except json.decoder.JSONDecodeError as e:
            print_info('bytes_to_dict', str(e))
            print("bytes_to_dict: ", translateds)
            return to_dict("bytes_to_dict: ", received)

        if type(json.loads(received.decode())) != dict:
            raise Exception("Parameter should be convertible to <class 'dict'> not to" + str(type(translateds)))

        return translateds


def dict_to_bytes(to_send):
    """
    Converts dict to bytes

    :param to_send:dict
    :return: bytes
    """
    if type(to_send) != dict:
        raise Exception("Parameter should be <class 'dict'> type not " + str(type(to_send)))
    return json.dumps(to_send).encode('utf-8')


def to_dict(key: str, value):
    """
    Returns dictionary from key and value

    :param key:
    :param value:
    :return: {key:value}
    """
    return dict({key: value})


def send(response: dict, connection_socket):
    try:
        connection_socket(dict_to_bytes(response))
        print_type("Network.send", response)
    except error as e:
        print_info("Network.send", e)


class NetworkServer:
    """Class responsible for what the server communicates"""

    def __init__(self, server_response_function, port=5910, packet_size=200):
        """
        Initializes a server listening to 5 connections
        
        :param server_response_function:    Response function receiving parameters (dict,socket) 
        :param port:  Server port: int default: 5910
        """""

        self.packet_size = packet_size
        self.server_response_function = server_response_function
        self.listening_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.listening_socket.bind(("", port))
        self.listening_socket.listen(5)
        print("Server started.")

    def threaded_client(self, socket_connection, ip_address):
        """Starts connection"""
        print_info("Connected to", ip_address)
        while True:
            try:
                client_response = socket_connection.recv(1024 * self.packet_size)
                if client_response:
                    print_type("Received", str(bytes_to_dict(client_response))[:100])
                    self.server_response_function(bytes_to_dict(client_response), socket_connection.sendall)
                else:
                    print_info("Disconnected from", ip_address)
                    break
            except (socket.error, KeyboardInterrupt) as e:
                self.server_response_function(to_dict("disconnected", ""), socket_connection.sendall)
                socket_connection.close()
                print(e)
                break

    def accept_connection(self):
        """Starts connection as new thread """
        socket_connection, ip_address = self.listening_socket.accept()
        start_new_thread(self.threaded_client, (socket_connection, ip_address,))


class NetworkClient:
    """
    Network client responsible for communicating with server.
    """

    def __init__(self, response_function, server_ip, server_port=5910, packet_size=200):
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
            self.connection_socket.connect(self.server_address)
            print("Connected to: ", self.server_address)
            start_new_thread(self.listen, ())
        except socket.error as e:
            print_type("NetworkClient.connect", e)

    def send(self, response: dict):
        try:
            self.connection_socket.sendall(dict_to_bytes(response))
            print_type("NetworkClient.send", str(response)[:100])
        except socket.error as e:
            print_type("NetworkClient.send", e)

    def listen(self):
        while True:
            try:
                data = self.connection_socket.recv(1024 * self.packet_size)
                if data:
                    self.response_function(bytes_to_dict(data))
                else:
                    print("Disconnected: ", str(self.server_address))
                    break
            except (socket.error, KeyboardInterrupt) as e:
                print_type("NetworkClient.listen", e)
                self.response_function(to_dict('reply', "Disconnected"))
                break
