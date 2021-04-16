import random


class Card:
    """Card having a suit and a number."""
    suitdict = {1: "Green", 2: "Yellow", 3: "Red", 4: "Rainbow"}

    def __init__(self, suit: int, number: int):
        # __init__ je konstruktor => on vytvara instanciu objektu premenna = Card()
        self.suit = suit
        self.number = number

    def __str__(self):
        # vracia objekt ako string = str(Card())
        if self.number == 11:
            return self.suitdict[int(self.suit)] + " Dragon"
        return self.suitdict[self.suit] + " " + str(self.number)

    def __eq__(self, other):
        # vracia rovnost kariet card == othercard
        if isinstance(other, Card):
            return self.suit == other.suit and self.number == other.number
        return NotImplemented

    def __gt__(self, other):
        # porovava velkost kariet podla cisel aj farby
        if isinstance(other, Card):
            if self.number == other.number and self.suit > other.suit:
                return True
            if self.number > other.number:
                return True
            return False
        return NotImplemented


class Player:
    def __init__(self, number):
        self.number = number
        self.hand = []

    def __str__(self):
        return "Player" + str(self.number)

    def add_card(self, card: Card):
        self.hand.append(card)

    def sort(self):
        pass


class Deck:
    """A deck capable of receiving shuffling and dealing cards."""
    deck = []

    def fill(self):
        suits = [1, 2, 3]
        numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        if not self.deck:
            for suit in suits:
                for number in numbers:
                    self.deck.append(Card(suit, number))
                    self.deck.append(Card(suit, number))
                self.deck.append(Card(suit, 11))
            self.deck.append(Card(4, 1))
            return True
        return False

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


def create_players(number: int):
    players = []
    for x in range(number):
        players.append(Player(x))
    return players


def deal(players: [Player], deck: Deck):
    for x in range(int(len(deck.deck) / len(players))):
        for player in players:
            player.add_card(deck.pull_card())
