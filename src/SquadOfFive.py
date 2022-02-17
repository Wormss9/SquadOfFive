import sys
import base64
from tkinter import filedialog
from tkinter import *

from _thread import start_new_thread
from PIL import Image, ImageTk

from logics.logics import GameClient, GameServer
from utils.utils import picture_to_string, string_to_image


chatReadText: Entry


def open_login_window():
    def command_button(ip, window):
        gameClient.settings.save_adress(ip)
        if not gameClient.settings.name:
            open_profile_window()
        response = gameClient.connect(ip)
        status_bar_text.set(response[1])
        if response[0]:
            window.destroy()

    global gameClient
    status_bar_text = StringVar()
    login_window = Toplevel(master)

    login_window.configure(bg='grey')
    login_window.iconbitmap('src/graphics/icon.ico')
    login_window.title("Connect")
    Label(login_window, text="Enter host ip_address:").grid(row=1, column=1)
    address = Entry(login_window)
    address.insert(END, str(gameClient.settings.adress))
    address.grid(row=1, column=2)
    login_button = Button(login_window, text="Connect")
    login_button['command'] = lambda: command_button(address.get(), login_window)
    login_button.grid(row=2, column=1, columnspan=2)
    status_bar = Label(login_window, textvariable=status_bar_text, relief='sunken')
    status_bar.grid(row=3, column=1, columnspan=2, sticky='we')


def open_profile_window():
    def command_button(name):
        gameClient.settings.save_name(name)
        gameClient.set_name(name)
        status_bar_text.set("Name changed")

    def picture_button():
        path = filedialog.askopenfilename(title="Select An Image", filetypes=(
            ("png files", "*.png"), ("gif files", "*.gif")))
        name_window.attributes('-topmost', 1)
        name_window.attributes('-topmost', 0)
        picture_string = picture_to_string(path)
        if picture_string:
            gameClient.settings.save_picture(picture_string)
            gameClient.set_picture(picture_string)
            new_photo_image = string_to_image(picture_string)
            player_picture.photo = ImageTk.PhotoImage(new_photo_image)
            player_picture.image = player_picture.photo
            player_picture.configure(image=player_picture.photo)
            status_bar_text.set("Picture changed")
        else:
            status_bar_text.set("Bad image")

    global gameClient
    status_bar_text = StringVar()
    name_window = Toplevel(master)

    name_window.configure(bg='grey')
    name_window.iconbitmap('src/graphics/icon.ico')
    name_window.title("Nick")
    Label(name_window, text="Enter nickname:").grid(row=1, column=1)
    address = Entry(name_window)
    address.insert(END, str(gameClient.settings.name))
    address.grid(row=1, column=2)

    name_button = Button(name_window, text="Change name")
    name_button['command'] = lambda: command_button(address.get())
    name_button.grid(row=2, column=1, columnspan=2)
    player_picture = Label(name_window)
    player_picture.grid(row=3, column=1, columnspan=2)
    photo_image = string_to_image(gameClient.settings.picture)
    player_picture.photo = ImageTk.PhotoImage(photo_image)
    player_picture.configure(image=player_picture.photo)
    picture_button = Button(name_window, text="Change picture", command=picture_button)
    picture_button.grid(row=4, column=1, columnspan=2)
    status_bar = Label(name_window, textvariable=status_bar_text, relief='sunken')
    status_bar.grid(row=5, column=1, columnspan=2, sticky='we')


def host_game():
    try:
        game_server = GameServer("Hosted server")
        start_new_thread(start_server, (game_server,))
        gameClient.connect("127.0.0.1")
    finally:
        gameClient.client_holder.status_bar['text'] = "Can't host server"


def start_server(game_server):
    while True:
        game_server.network.accept_connection()


class ClientHolder:
    chatTextArea: Label
    status_bar: Label
    chat = ""
    card_on_table: Label
    cards_on_hand: Label
    hand = []
    table = []
    add_line_to_chat = ""
    selected_cards = []
    player_frame_list = []
    player_details_list = []

    def set_players(self, number, name, card_num):
        self.player_details_list[number][0]['text'] = name
        self.player_details_list[number][2]['text'] = str(card_num)

    def show_cards_hand(self):
        for child in self.cards_on_hand.winfo_children():
            child.destroy()
        counter = 1
        self.selected_cards.clear()
        for card_p in self.hand:
            card_with_picture = [card_p, IntVar(value=-1), ImageTk.PhotoImage(
                Image.open("src/graphics/cards/" + card_p.image + ".png").resize((card_width, card_height))),
                                 ImageTk.PhotoImage(
                                     Image.open("src/graphics/cards/" + card_p.image + "s.png").resize(
                                         (card_width, card_height)))]
            self.selected_cards.append(card_with_picture)
            self.selected_cards[counter - 1].append(Checkbutton(self.cards_on_hand,
                                                                image=self.selected_cards[counter - 1][2],
                                                                selectimage=self.selected_cards[counter - 1][3],
                                                                indicatoron=False,
                                                                onvalue=counter - 1,
                                                                offvalue=-1,
                                                                variable=self.selected_cards[counter - 1][1],
                                                                bg='grey20'))
            self.selected_cards[counter - 1][4].grid(row=1, column=counter)
            counter += 1

    def show_card_table(self):
        global card_width
        global card_height
        for child in self.card_on_table.winfo_children():
            child.destroy()
        counter = 1
        for card_p in self.table:
            card_image = Label(self.card_on_table, bg='grey30')
            card_image.grid(row=1, column=counter)
            counter += 1
            photo = ImageTk.PhotoImage(
                Image.open("src/graphics/cards/" + card_p.image + ".png").resize((card_width, card_height)))
            card_image.configure(image=photo)
            card_image.image = photo

    def set_picture(self, number, picture):
        photo_image = ImageTk.PhotoImage(Image.frombytes('RGBA', (100, 100), base64.b64decode(picture)))
        self.player_details_list[number - 1][1].photo = photo_image
        self.player_details_list[number - 1][1].configure(image=photo_image)

    def set_turn(self, turn):
        for frame in self.player_frame_list:
            frame.configure(bg='grey20')
        self.player_frame_list[turn - 1].config(bg="gold2")
        for player_details in self.player_details_list:
            for player_detail in player_details:
                player_detail.configure(bg='grey20')
        for player_detail in self.player_details_list[turn - 1]:
            player_detail.configure(bg='gold2')


class MainMenu(Menu):
    submenu: Menu

    def __init__(self, parent):
        Menu.__init__(self, parent)
        self.subMenu = Menu(self, tearoff=0)
        self.subMenu.add_command(label="Connect", command=open_login_window)
        self.subMenu.add_command(label="Change profile", command=open_profile_window)
        self.subMenu.add_command(label="Host", command=host_game)
        self.add_cascade(label="Game", menu=self.subMenu)


class PlayerZone(Label):
    default_player_photo: PhotoImage
    player_zones = []
    player_frame = []

    def __init__(self, parent, row, column):
        Label.__init__(self, parent)
        self.grid(row=row, column=column)
        for x in range(4):
            self.player_frame.append(Frame(self, background="grey20"))
            self.player_frame[x].grid(column=x + 1, row=1)
            self.player_zones.append(
                [Label(self.player_frame[x], text="Player" + str(x + 1), bg='grey20', fg='lime'),
                 Label(self.player_frame[x], bg='grey20', fg='lime'),
                 Label(self.player_frame[x], text=16, bg='grey20', fg='lime')])

        self.default_player_photo = string_to_image(picture_to_string('src/graphics/defaultPlayer.png'))
        self.default_player_photo = ImageTk.PhotoImage(self.default_player_photo)
        print(type(self.default_player_photo))
        for x in range(4):
            for y in range(3):
                self.player_zones[x][y].grid(row=y + 1)
                if y == 1:
                    self.player_zones[x][y].photo = self.default_player_photo
                    self.player_zones[x][y].image = self.default_player_photo
                    self.player_zones[x][y].configure(image=self.default_player_photo)

        client_holder.player_frame_list = self.player_frame
        client_holder.player_details_list = self.player_zones


class TableZone(Label):
    # todo cards
    def __init__(self, parent, row, column):
        Label.__init__(self, parent, bg='grey30')
        self.grid(row=row, column=column)
        client_holder.card_on_table = self


class DeckZone(Label):
    def __init__(self, parent, row, column):
        Label.__init__(self, parent, bg='grey30')
        self.grid(row=row, column=column)
        client_holder.cards_on_hand = self


class DeckButtonZone(Label):

    def pass_button(self):
        gameClient.send({"pass": ""})

    def play_button(self):
        play = []
        for card_block in client_holder.selected_cards:
            if card_block[1].get() >= 0:
                play.append(card_block[0].to_list())
        gameClient.send({"play": play})

    def __init__(self, parent, row, column):
        Label.__init__(self, parent, relief='sunken', bg='grey30', fg='lime')
        self.grid(row=row, column=column, sticky='we')
        self.deckButton_zone_passButton = Button(self, text="Pass", command=self.pass_button, bg='grey20', fg='lime')
        self.deckButton_zone_passButton.grid(row=1, column=1)
        self.deckButton_zone_playButton = Button(self, text="Play", command=self.play_button, bg='grey20', fg='lime')
        self.deckButton_zone_playButton.grid(row=1, column=2, sticky='we')


class StatusBar(Label):
    def __init__(self, parent, row, column):
        Label.__init__(self, parent, text="Welcome", relief='sunken', bg='grey20', fg='lime')
        self.grid(row=row, column=column, columnspan=2, sticky='we')
        client_holder.status_bar = self


class ChatTextFrame(Frame):
    def __init__(self, parent):
        Frame.__init__(self, parent)
        self.canvas = Canvas(self, borderwidth=0, bg='grey20')
        self.frame = Frame(self.canvas, bg='grey20')

        self.vsb = Scrollbar(self, orient="vertical", command=self.canvas.yview)
        self.canvas.configure(yscrollcommand=self.vsb.set)

        self.vsb.pack(side="right", fill="y")
        self.canvas.pack(side="left", fill="both", expand=True)
        self.canvas.create_window((4, 3), window=self.frame, anchor="nw",
                                  tags="self.frame")

        self.frame.bind("<Configure>", self.on_frame_configure)

        self.textArea = Label(self.frame, bg='grey20', fg='lime')
        self.textArea.grid(sticky=W)

    def add_line_to_chat(self, text):
        self.textArea['text'] += text + "\n"

    def on_frame_configure(self, event):
        self.canvas.configure(scrollregion=self.canvas.bbox("all"))


class ChatZone(Label):
    example: ChatTextFrame
    chatReadText: Entry
    chatInput: Label
    chatSendButton: Button
    chatReportButton: Button

    def send_message(self):
        gameClient.send({"chat": self.chatReadText.get()})

    def send_report(self):
        gameClient.send({"report": ""})

    def __init__(self, parent, row, column):
        Label.__init__(self, parent, bg='grey30')
        self.grid(row=row, column=column, rowspan=4)

        self.example = ChatTextFrame(self)
        client_holder.add_line_to_chat = self.example.add_line_to_chat
        self.example.pack(side="top", fill="both", expand=True)
        self.chatInput = Label(self, bg='grey30')
        self.chatReadText = Entry(self.chatInput)

        self.chatSendButton = Button(self.chatInput, text="Send", bg='grey20', fg='lime', command=self.send_message)
        self.chatReportButton = Button(self.chatInput, text="Report", bg='grey20', fg='lime', command=self.send_report)

        self.chatInput.pack(side="bottom")
        self.chatReadText.grid()
        self.chatSendButton.grid()
        self.chatReportButton.grid()


class MainWindow(Tk):
    main_menu: MainMenu
    player_zone: PlayerZone
    table_zone: TableZone
    deck_zone: DeckZone
    deck_button_zone: DeckButtonZone
    status_bar: StatusBar
    chat_zone: ChatZone

    def __init__(self):
        Tk.__init__(self)
        self.title("Squad of Five")
        self.configure(bg='grey30')
        self.iconbitmap('src/graphics/icon.ico')

        self.main_menu = MainMenu(self)
        self.config(menu=self.main_menu)

        self.player_zone = PlayerZone(self, 1, 1)
        self.table_zone = TableZone(self, 2, 1)
        self.deck_zone = DeckZone(self, 3, 1)
        self.deck_button_zone = DeckButtonZone(self, 4, 1)
        self.status_bar = StatusBar(self, 5, 1)
        self.chat_zone = ChatZone(self, 1, 2)


card_height = 100
card_width = 70

client_holder = ClientHolder()
master = MainWindow()

if len(sys.argv) == 1:
    gameClient = GameClient(client_holder, "")
else:
    gameClient = GameClient(client_holder, str(sys.argv[1]))
mainloop()
