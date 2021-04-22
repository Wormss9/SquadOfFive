# from network import *
from logics import *

game = GameServer()
network = NetworkServer(player_list=game.players, server_response_function=game.respond)
while True:
    network.accept_connection()
