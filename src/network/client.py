import socket
from _thread import start_new_thread, error

from utils.utils import bytes_to_dict, dict_to_bytes, print_type, to_dict

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
                    start_new_thread(self.response_function, (bytes_to_dict(data),))
                    # self.response_function(bytes_to_dict(data))
                else:
                    print("Disconnected: ", str(self.server_address))
                    break
            except (socket.error, KeyboardInterrupt) as e:
                print_type("NetworkClient.listen", e)
                self.response_function([to_dict('reply', "Disconnected")])
                break
