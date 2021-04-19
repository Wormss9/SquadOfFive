# from network import *
from logics import *

game = GameServer()
server = Server(game=game)
while True:
    server.accept()
