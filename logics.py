import json
import random
from network import NetworkServer, NetworkClient, to_dict
from _thread import start_new_thread, error

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
    suitDict = {1: "r", 2: "g", 3: "b", 4: "w"}

    def __init__(self, suit: int, number: int):
        # __init__ je konstruktor => on vytvara instanciu objektu premenna = Card()
        self.suit = suit
        self.number = number
        self.image = self.suitDict[self.suit] + str(number).zfill(2)

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

    def connect(self, name: str, connection):
        self.name = name
        self.connection = connection
        self.connected = True

    def disconnect(self):
        self.connected = False

    def add_card(self, card: Card):
        self.hand.append(card)

    def sort_cards(self):
        self.hand.sort()

    def hand_to_list(self):
        hand_list = []
        for card in self.hand:
            hand_list.append([card.suit, card.number])
        return hand_list

    def is_connected(self, connection_to_player=False):
        if hasattr(self, 'connection') and self.connected:
            if connection_to_player:
                return connection_to_player == self.connection
            return True
        return False


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
        for key in data:
            self.process_respondable(key, data.get(key))

    def process_respondable(self, key, word):
        print("Processing: ", key, str(word).replace('\n', ' '))

        if key == 'chat':
            self.client_holder.chat += word
            self.client_holder.add_line_to_chat(word)

        if key == 'reply':
            self.client_holder.status_bar['text'] = word

        if key == 'hand' or key == 'table':
            self.client_holder.hand = []
            for card_list in word:
                self.client_holder.hand.append(Card(card_list[0], card_list[1]))
            self.client_holder.show_cards(key)


        else:
            print('Unknown: "', key, '"')


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
        for key in data:
            self.process_respondable(key, data.get(key), connection_to_player)

    def process_respondable(self, key, word, connection_to_player):
        print("Processing: ", key, str(word).replace('\n', ' '), connection_to_player)

        if key == 'name':
            for player in self.players:
                if not player.connected and hasattr(player, 'name') and player.name == word:
                    player.connect(word, connection_to_player)
                    print(word + " reconnected.")
                    self.network.send(
                        {'connection': True, 'reply': word + ' reconnected to ' + self.name, 'chat': self.chat,
                         'hand': player.hand_to_list()}, player.connection)
                    return
            for player in self.players:
                if not player.connected and not hasattr(player, 'name'):
                    player.connect(word, connection_to_player)
                    print(word + " connected.")
                    self.network.send(
                        {'connection': True, 'reply': word + ' connected to ' + self.name, 'chat': self.chat,
                         'hand': player.hand_to_list()}, player.connection)
                    return
            self.network.send({connection: False,
                               'reply': data.name + ' is full'})


        elif key == 'chat':
            name = "Anon"
            for player in self.players:
                if player.is_connected(connection_to_player):
                    name = player.name
            self.chat += name + ": " + str(word).replace('\n', '    \n') + '\n'
            for player in self.players:
                if player.is_connected():
                    self.network.send(to_dict("chat", "\n" + name + ": " + word), player.connection)


        elif key == 'disconnected':
            for player in self.players:
                if hasattr(player, 'connection') and player.connected and player.connection == connection_to_player:
                    player.disconnect()

        elif key == 'report':
            for player in self.players:
                if player.is_connected(connection_to_player):
                    name = player.name
                for player in self.players:
                    if player.is_connected():
                        self.network.send(to_dict("reply", name + " reported himself"), player.connection)
        else:
            print("unknown ", key, " ", word)


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
