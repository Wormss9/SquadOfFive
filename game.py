import random
import struct
import json

from networking import *

playPowerDic = {1: "single",
                2: "pair",
                3: "three of a kind",
                4: "straight suite",
                5: "flash",
                6: "full house",
                7: "straight flush",
                8: "gang of four",
                9: "gang of five",
                10: "gang of six",
                11: "gang of seven"
                }


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
    def __init__(self, conn="local"):
        self.name = ""
        self.hand = []
        self.conn = conn
        self.connected = False

    def __str__(self):
        return "Player" + self.name

    def connect(self, name: str):
        self.name = name
        self.connected = True

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


class Play:
    def __init__(self, cards: [Card]):
        self.cards = cards

    def value(self):
        # single
        if len(self.cards) == 1:
            return [self.cards[0].number, 0, self.cards[0].color, 0, 1]
        # pair
        if len(self.cards) == 2 and self.cards[0].number == self.cards[1].number:
            return [card.number, 0, 1]
        #
        return False

    def __gt__(self, other):
        if power(self.cards) and power(other.cards):
            if len(self.cards) == len(other.cards):
                if len(self.cards) == 1:
                    return self.cards[0] > other.cards[0]
        return False


class GameState:
    def __init__(self):
        self.ip = ""
        self.name = ""
        self.settings = Settings()

    def connect(self, ip, window):
        try:
            self.connection = Client(server=ip)
            print(json.dumps({"name": self.name}))
            self.connection.send(json.dumps({"name": self.name}))
            self.ip = ip
        except:
            return "Connection failed"
        window.destroy()
        return "Connected"

    def set_name(self, name, window):
        self.name = name
        window.destroy()
        return "Name changed to " + name


class GameServer:
    def __init__(self):
        self.ip = ""
        self.name = ""
        self.players = [Player(), Player(), Player(), Player()]

    def connect(self, ip, window):
        try:
            self.connection = Client(server=ip)
            self.ip = ip
        except:
            return "Connection failed"
        window.destroy()
        return "Connected"

    def set_name(self, name, window):
        self.name = name
        window.destroy()
        return "Name changed to " + name

    def reply(self, data):
        if 'name' in data:
            for player in self.players():
                if player.connected == False:
                    player.connected = True
                    player.name = data[1]
                    return {"connection": True,
                            "reply": data.name + " connected"}
            return {connection: False,
                    "reply": data.name + "Game is full"}


class Settings:
    def __init__(self):
        self.name = ""
        self.adress = ""
        try:
            with open('settings.txt') as file:
                setting = json.load(file)
                self.name = setting['name']
                self.adress = setting['adress']
        except error:
            print(str(error))

    def save_adress(self, adress):
        self.adress = adress
        self.saveToFile()

    def save_name(self, name):
        self.name = name
        self.saveToFile()

    def saveToFile(self):
        data = {"name": self.name,
                "adress": self.adress}
        with open('settings.txt', 'w') as outfile:
            json.dump(data, outfile)


def create_players(number: int):
    players = []
    for x in range(number):
        players.append(Player(x))
    return players


def deal(players: [Player], deck: Deck):
    for x in range(int(len(deck.deck) / len(players))):
        for player in players:
            player.add_card(deck.pull_card())
