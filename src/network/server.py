import socket
import json
from _thread import start_new_thread, error

from utils.utils import bytes_to_dict, print_info, print_type, to_dict

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
                self.server_response_function([to_dict("disconnected", "")], socket_connection.sendall)
                socket_connection.close()
                print(e)
                break

    def accept_connection(self):
        """Starts connection as new thread """
        socket_connection, ip_address = self.listening_socket.accept()
        start_new_thread(self.threaded_client, (socket_connection, ip_address,))
