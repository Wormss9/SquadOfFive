import random


class Card:
    """Card having a suit and a number."""

    def __init__(self, suit, number):
        # __init__ je konstruktor => on vytvara instanciu objektu
        self.suit = suit
        self.number = number


class Deck:
    """A deck capable of receiving shuffling and dealing cards."""
    deck = []

    def fill_poker(self):
        suits = ['Spade', 'Diamond', 'Heart', 'Club']
        numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A']
        for suit in suits:
            for number in numbers:
                self.deck.append(Card(suit, number))

    def add_card(self, card: Card):
        """Adds a card"""
        return self.deck.append(card)

    def shuffle(self):
        """Shuffles a non empty deck"""
        if (self.deck):
            random.shuffle(self.deck)
            return True
        else:
            return False

    def pull_card(self):
        """Takes a card from the deck and returns it"""
        return self.deck.pop()


deck = Deck()
deck.fill_poker()
deck.shuffle()
for card in deck.deck:
    print(card.number + " of " + card.suit + "s")
