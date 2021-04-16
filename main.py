from game import *

# from graphics import *

# to do => napisat hru

# Create and prepare deck
deck = Deck()
deck.fill()
deck.shuffle()

# Create players
players = create_players(4)
# Deal cards
deal(players, deck)

players[0].hand.sort()
for card in players[0].hand:
    print(str(card))
