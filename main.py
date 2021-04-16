from game import *
# from graphics import *

# to do => napisat hru

#Create and prepare deck
deck = Deck()
deck.fill_squad()
print(len(deck.deck))
#deck.shuffle()

#Create players
players=create_players(4)
#Deal cards
test=[players[0],players[1]]
deal(test, deck)


print(len(players[0].hand))
print(len(players[1].hand))
print(len(players[2].hand))
print(len(players[3].hand))

print(str(players[0]))
for card in players[0].hand:
    print(str(card))