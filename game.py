import random


class Card:
    """Card having a suit and a number."""
    def __init__(self, suit, number):
        # __init__ je konstruktor => on vytvara instanciu objektu
        self.suit = suit
        self.number = number
    def __str__(self):
        return self.suit + " " + self.number

class Player:
    hand=[]
    def __init__(self, number):
        self.number=number
    def __str__(self):
        return "Player"+str(self.number)
    def add_card(self,card:Card):
        self.hand.append(card)
    def sort(self):
        pass


class Deck:
    """A deck capable of receiving shuffling and dealing cards."""
    deck = []

    def fill_poker(self):
        suits = ['Spade', 'Diamond', 'Heart', 'Club']
        numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A']
        if not self.deck:
            for suit in suits:
                for number in numbers:
                    self.deck.append(Card(suit, number))
            return True
        return False

    def fill_squad(self):
        suits = ['Red', 'Yellow', 'Green']
        numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10']
        if not self.deck:
            for suit in suits:
                for number in numbers:
                    self.deck.append(Card(suit, number))
                    self.deck.append(Card(suit, number))
                self.deck.append(Card(suit, 'D'))
            self.deck.append(Card('Rainbow', '1'))
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

def create_players(number:int):
    players=[]
    for x in range(number):
        players.append(Player(x))
        print("number "+str(x))
    return players

def deal(players:[Player],deck:Deck):
    for x in range(int(len(deck.deck)/len(players))):
        for player in players:
            card=deck.pull_card()
            print(str(player)+" got a "+str(card))
            player.add_card(card)