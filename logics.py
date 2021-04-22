import random
import struct
from network import *

playPowerDic = {1: "single",
                2: "pair",
                3: "three of a kind",
                4: "straight suite",
                5: "flush",
                6: "full house",
                7: "straight flush",
                8: "gang of four",
                9: "gang of five",
                10: "gang of six",
                11: "gang of seven"
                }


class Card:
    """Card having a suit and a number."""
    suitDict = {1: "Red", 2: "Green", 3: "Blue", 4: "White"}

    def __init__(self, suit: int, number: int):
        # __init__ je konstruktor => on vytvara instanciu objektu premenna = Card()
        self.suit = suit
        self.number = number

    def __str__(self):
        """Returns name of card"""
        # vracia objekt ako string = str(Card())
        if self.number == 11:
            return self.suitDict[int(self.suit)] + " Diamond"
        return self.suitDict[self.suit] + " " + str(self.number)

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
    """Player as recognized by the server"""

    def __init__(self):
        self.name: str
        self.hand = []
        self.connection: socket.socket
        self.connected = False

    def __str__(self):
        return self.name

    def connect(self, name: str):
        self.name = name
        self.connected = True

    def disconnect(self):
        # todo Actually call this function
        self.connected = False

    def add_card(self, card: Card):
        self.hand.append(card)

    def sort_cards(self):
        self.hand.sort()


class Deck:
    """A random card deck."""

    def __init__(self, players: [Player]):
        self.players = players
        self.deck = []
        self.fill()
        self.shuffle()
        self.deal()

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

    def deal(self):
        for x in range(int(len(self.deck) / len(self.players))):
            for player in self.players:
                player.add_card(self.pull_card())


class Play:
    """Defines the cards on the table and the ones you are putting on it."""

    def __init__(self, cards: [Card]):
        self.cards = cards

    def value(self):
        self.cards.sort()
        flush = False
        straight = False
        """Returns the value of the hand. Usefull for comparison. Needed for first play."""
        # finding out wether you have suit or a flush because they have no expressive value alone
        if len(self.cards) == 5:
            if self.cards[0].suit == self.cards[1].suit == self.cards[2].suit == self.cards[3].suit == self.cards[
                4].suit:
                flush = True
            if self.cards[4].number == self.cards[3].number + 1 == self.cards[2].number + 2 == self.cards[
                1].number + 3 == self.cards[0].number + 4:
                straight = True
        # 1: "single"
        if len(self.cards) == 1:
            return 1
        # 2: "pair"
        if len(self.cards) == 2 and self.cards[0].number == self.cards[1].number:
            return 2
        # 3: "three of a kind"
        if len(self.cards) == 3 and self.cards[0].number == self.cards[1].number == self.cards[2].number:
            return 3
        # 4: "straight"
        if straight and not flush:
            return 4
        # 5: "flush"
        if flush and not straight:
            return 5
        # todo 6: "full house"

        # 7: "straight flush"
        if flush and straight:
            return 7
        # 8: "gang of four"
        if len(self.cards) == 4:
            pass
        # 9: "gang of five"
        if len(self.cards) == 5:
            pass
        # 10: "gang of six"
        if len(self.cards) == 6:
            pass
        # 11: "gang of seven"
        if len(self.cards) == 7:
            pass
        return 0

    def __gt__(self, other):
        my_case = puttable_on_empty_table(self)
        other_case = puttable_on_empty_table(other)
        if my_case == other_case == 1:
            return self.cards[0] > other.cards[0]
        if my_case == other_case == 2:
            # not comparing all card by color
            return self.cards[0] > other.cards[0]
        if my_case == other_case == 3:
            # not comparing all card by color
            return self.cards[0] > other.cards[0]
        # todo
        return False


class GameClient:
    def __init__(self, client_holder):
        self.settings = Settings()
        self.ip = self.settings.adress
        self.name = self.settings.name
        self.hand = []
        self.connection: NetworkClient
        self.client_holder = client_holder

    def connect(self, ip):
        try:
            self.connection = NetworkClient(response_function=self.respond, server_ip=ip)
            self.connection.connect()
            self.connection.send(to_dict('name', self.name))
        except error as e:
            print_type("GameClient.connect", e)
            return (False, "Connection failed")
        return (True, "Connected")

    def send(self, message: dict):
        self.connection.send(message)

    def set_name(self, name):
        self.name = name
        return (True, "Name changed to " + name)

    def respond(self, data):
        reply = {}
        for key in data:
            reply.update(self.process_respondable(key, data.get(key)))
        return reply

    def process_respondable(self, key, word):
        print("Processing: ", key, str(word).replace('\n', ' '))
        if key == "chat":
            self.client_holder.chatTextArea['text'] = word
            return {}
        else:
            return {}


class GameServer:
    def __init__(self, name):
        self.network = NetworkServer(server_response_function=self.respond)
        self.name = name
        self.players = [Player(), Player(), Player(), Player()]
        self.chat = ""
        Deck(self.players)
        for player in self.players:
            player.sort_cards()

    def respond(self, data, connection_to_player):
        reply = {}
        for key in data:
            reply.update(self.process_respondable(key, data.get(key), connection_to_player))
        return reply

    def process_respondable(self, key, word, connection_to_player):
        print("Processing: ", key, str(word).replace('\n', ' '), connection_to_player)
        if key == "name":
            for player in self.players:
                if player.connected == False:
                    player.connected = True
                    player.name = word
                    player.connection = connection_to_player
                    return {'connection': True,
                            'reply': word + ' connected'}
            return {connection: False,
                    'reply': data.name + 'Game is full'}
        elif key == 'chat':
            name = "Anon"
            for player in self.players:
                if hasattr(player, 'connection') and player.connected and player.connection == connection_to_player:
                    name = player.name
            self.chat += name + ": " + str(word).replace('\n', '    \n') + '\n'
            for player in self.players:
                if hasattr(player, 'connection') and player.connected:
                    print("Sending to: ", player.name, " ; ", self.chat.replace('\n', ' '))
                    self.network.send(to_dict("chat", self.chat), player.connection)
            return {}
        elif key == 'disconnected':
            for player in self.players:
                if hasattr(player, 'connection') and player.connected and player.connection == connection_to_player:
                    player.connected = False
        else:
            return {}


class Settings:
    def __init__(self):
        self.name = ""
        self.adress = ""
        try:
            with open('settings.txt') as file:
                setting = json.load(file)
                self.name = setting['name']
                self.adress = setting['address']
        except:
            pass

    def save_adress(self, adress):
        self.adress = adress
        self.saveToFile()

    def save_name(self, name):
        self.name = name
        self.saveToFile()

    def saveToFile(self):
        data = {"name": self.name,
                "address": self.adress}
        with open('settings.txt', 'w') as outfile:
            json.dump(data, outfile)
