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

for player in players:
    player.hand.sort()
