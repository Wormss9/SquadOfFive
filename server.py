# from network import *
from logics import *

game = GameServer()
server = Server(game_server_logic=game)
while True:
    server.accept()
