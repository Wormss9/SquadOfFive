from game import *
from graphics import *

#to do => napisat hru

deck = Deck()
deck.fill_poker()
deck.shuffle()
for card in deck.deck:
    print(card.number + " of " + card.suit + "s")