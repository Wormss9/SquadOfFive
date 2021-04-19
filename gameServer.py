#from networking import *
from gameLogics import *

game = GameServer()
server = Server(game=game)
while True:
    server.accept()
