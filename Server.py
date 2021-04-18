from networking import *
from game import *

game=GameServer()
server = Server(game=game)
while True:
    server.accept()
