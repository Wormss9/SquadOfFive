from logics.logics import GameServer

game = GameServer("TestServer")
while True:
    game.network.accept_connection()
