import json
import random
from network import NetworkServer, NetworkClient, to_dict, send
from _thread import start_new_thread, error
from PIL import Image, ImageTk, UnidentifiedImageError
from tkinter import PhotoImage
import base64

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

photo_size = 100


def picture_to_string(path):
    try:
        a = Image.open(path).convert('RGBA')
    except AttributeError:
        return False
    except UnidentifiedImageError:
        return False
    b = a.resize((100, 100))
    c = b.tobytes()
    d = base64.b64encode(c)
    e = d.decode()
    return e


def string_to_image(photo_string):
    a = photo_string.encode()
    b = base64.b64decode(a)
    c = Image.frombytes('RGBA', (100, 100), b)
    return c


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

    def to_list(self):
        return [self.suit, self.number]


class Player:
    """Player as recognized by the server"""

    def __init__(self, turn_number, server_turn):
        self.name: str
        self.hand = []
        self.connection: socket.socket
        self.connected = False
        self.turn_number = turn_number
        self.server_turn = server_turn
        self.picture = picture_to_string('graphics/defaultPlayer.png')

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

    def my_turn(self):
        return self.turn_number == self.server_turn


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
        my_case = self.value()
        other_case = other.value()
        if my_case > other_case and (other_case == 0 or (4 >= other_case > 7 and 4 > my_case > 7) or my_case >= 8):
            return True
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
        self.picture = self.settings.picture
        self.hand = []
        self.connection: NetworkClient
        self.client_holder = client_holder
        self.status_bar: Label

    def connect(self, ip):
        try:
            self.connection = NetworkClient(response_function=self.respond, server_ip=ip)
            self.connection.connect()
            self.connection.send({'name': self.name, 'picture': self.picture})
        except error as e:
            print("GameClient.connect ", e)
            return (False, "Connection failed")
        return (True, "Connected")

    def send(self, message: dict):
        try:
            self.connection.send(message)
        except AttributeError:
            self.client_holder.status_bar['text'] = "Not connected."

    def set_name(self, name):
        self.name = name
        return (True, "Name changed to " + name)

    def respond(self, data):
        for dic in data:
            for key in dic:
                if type(dic) == dict:
                    self.process_respondable(key, dic.get(key))
                else:
                    raise Exception(
                        "Data must be a list of dictionaries \n Data:" + str(data) + "\n Dict:" + str(type(dic)) + str(
                            dic))

    def process_respondable(self, key, word):
        print("Processing: ", key, str(word).replace('\n', ' ')[:50])

        if key == 'chat':
            self.client_holder.chat += word
            self.client_holder.add_line_to_chat(word)

        elif key == 'reply':
            self.client_holder.status_bar['text'] = word

        elif key == 'hand':
            self.client_holder.hand = []
            for card_list in word:
                self.client_holder.hand.append(Card(card_list[0], card_list[1]))
            self.client_holder.show_cards_hand()

        elif key == 'table':
            self.client_holder.table = []
            for card_list in word:
                self.client_holder.table.append(Card(card_list[0], card_list[1]))
            self.client_holder.show_card_table()

        elif key == 'players':
            counter = 0
            for player_list in word:
                self.client_holder.set_players(counter, player_list[0], player_list[1])

        elif key == 'picture':
            print("number" + str(word[0]))
            print("picture in base64" + str(word[1]))
            self.client_holder.set_picture(word[0], word[1])

        elif key == 'turn':
            self.client_holder.set_turn(word)

        else:
            print('Unknown: "', key, '"')


class GameServer:
    def __init__(self, name):
        self.network = NetworkServer(server_response_function=self.respond)
        self.name = name
        self.turn = random.randrange(1, 5)
        self.players = []
        for x in range(4):
            self.players.append(Player(x + 1, self.turn))
        self.chat = ""
        Deck(self.players)
        self.table = []
        self.pass_counter = 0
        # todo intagrate this
        for player in self.players:
            player.sort_cards()

    def restart(self, turn):
        self.turn = turn
        for player in self.players:
            player.hand = []
        Deck(self.players)
        self.table = []
        for player in self.players:
            player.sort_cards()

    def next_turn(self):
        if self.turn == 4:
            self.turn = 1
        else:
            self.turn += 1
        if hasattr(self.players[self.turn - 1], 'name'):
            self.send_to_all(to_dict('reply', "It's " + self.players[self.turn - 1].name + "'s turn."))
        else:
            self.send_to_all(to_dict('reply', "It's Player" + str(self.turn) + "'s turn."))

    def passs(self):
        self.pass_counter += 1
        if self.pass_counter == 4:
            return True
        return False

    def respond(self, data, connection_to_player):
        for dic in data:
            print("bock di:", type(dic), dic)
            for key in dic:
                self.process_respondable(key, dic.get(key), connection_to_player)

    def players_name_list(self):
        name_list = []
        for player in self.players:
            if hasattr(player, 'name'):
                name_list.append([player.name, len(player.hand)])
            else:
                break
        return name_list

    def send_to_all(self, message):
        for player in self.players:
            if player.is_connected():
                send(message, player.connection)

    def process_respondable(self, key, word, connection_to_player):
        print("Processing: ", key, str(word).replace('\n', ' '), str(connection_to_player)[-19:-1])
        if key == 'name':
            for player in self.players:
                if not player.connected and hasattr(player, 'name') and player.name == word:
                    player.connect(word, connection_to_player)
                    print(word + " reconnected.")
                    return_table = []
                    for card in self.table:
                        return_table.append([card.suit, card.number])
                    send({'connection': True, 'reply': word + ' reconnected to ' + self.name, 'chat': self.chat,
                          'hand': player.hand_to_list(), 'players': self.players_name_list(), 'table': return_table,
                          'turn': self.turn}, player.connection)
                    return
                if not player.connected and not hasattr(player, 'name'):
                    player.connect(word, connection_to_player)
                    print(word + " connected.")
                    return_table = []
                    for card in self.table:
                        return_table.append([card.suit, card.number])
                    send({'connection': True, 'reply': word + ' connected to ' + self.name, 'chat': self.chat,
                          'hand': player.hand_to_list(), 'players': self.players_name_list(), 'table': return_table,
                          'turn': self.turn}, player.connection)
                    return
            send({'connection': False, 'reply': data.name + ' is full'})

        elif key == 'picture':
            for player in self.players:
                if player.is_connected(connection_to_player):
                    player.picture = word
                    self.send_to_all(to_dict('picture', [player.turn_number, player.picture]))


        elif key == 'chat':
            for player in self.players:
                if player.is_connected(connection_to_player):
                    self.chat += player.name + ": " + word + '\n'
                    self.send_to_all(to_dict("chat", player.name + ": " + word))

        elif key == 'disconnected':
            for player in self.players:
                if player.is_connected(connection_to_player):
                    player.disconnect()
                    self.send_to_all(to_dict('reply', player.name + " disconnected."))

        elif key == 'report':
            for player in self.players:
                if player.is_connected(connection_to_player):
                    self.send_to_all(to_dict("reply", player.name + " reported himself"))

        elif key == 'pass':
            for player in self.players:
                if player.is_connected(connection_to_player) and player.my_turn():
                    if self.passs():
                        self.table = []
                        self.send_to_all(to_dict("table", self.table))
                    self.next_turn()
                    self.send_to_all(to_dict('turn', self.turn))


        elif key == 'play':
            for player in self.players:
                if player.is_connected(connection_to_player) and player.my_turn():
                    play_list = []
                    for card_list in word:
                        play_list.append(Card(card_list[0], card_list[1]))
                    play = Play(play_list)
                    if play.value() and play > Play(self.table):
                        self.table = play_list.copy()
                        self.next_turn()
                        for player in self.players:
                            if player.is_connected(connection_to_player):
                                player.hand = [card for card in player.hand if
                                               not card in play_list or play_list.remove(card)]
                                send({'hand': player.hand_to_list()}, connection_to_player)
                                return_table = []
                                for card in self.table:
                                    return_table.append([card.suit, card.number])
                                self.send_to_all({"table": return_table, 'players': self.players_name_list()})

                    else:
                        for player in self.players:
                            if player.is_connected(connection_to_player):
                                send(to_dict("reply", "Bad play"), player.connection)
                else:
                    if player.is_connected(connection_to_player):
                        send(to_dict("reply", "Not your turn"), player.connection)


        else:
            print("Unknown key ", key, " : ", word)


class Settings:
    def __init__(self):
        self.name = ""
        self.adress = ""
        self.picture = picture_to_string('graphics/defaultPlayer.png')
        try:
            with open('settings.txt') as file:
                setting = json.load(file)
                self.name = setting['name']
                self.adress = setting['address']
                self.picture = setting['picture']
        except:
            pass

    def save_adress(self, adress):
        self.adress = adress
        self.saveToFile()

    def save_name(self, name):
        self.name = name
        self.saveToFile()

    def save_picture(self, picture):
        self.picture = picture
        self.saveToFile()

    def saveToFile(self):
        data = {"name": self.name,
                "address": self.adress,
                "picture": self.picture}
        with open('settings.txt', 'w') as outfile:
            json.dump(data, outfile)
