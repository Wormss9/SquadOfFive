from tkinter import *
from logics import GameServer, GameClient, Card
from _thread import start_new_thread
from PIL import Image, ImageTk


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
    player_show_list = []

    def show_cards_hand(self):
        for child in self.cards_on_hand.winfo_children():
            child.destroy()
        counter = 1
        self.selected_cards.clear()
        for card_p in self.hand:
            card_with_picture = [card_p, IntVar(value=-1), ImageTk.PhotoImage(
                Image.open("graphics/cards/" + card_p.image + ".png").resize((card_width, card_height))),
                                 ImageTk.PhotoImage(
                                     Image.open("graphics/cards/" + card_p.image + "s.png").resize(
                                         (card_width, card_height)))]
            self.selected_cards.append(card_with_picture)
            self.selected_cards[counter - 1].append(Checkbutton(self.cards_on_hand,
                                                                image=self.selected_cards[counter - 1][2],
                                                                selectimage=self.selected_cards[counter - 1][3],
                                                                indicatoron=False,
                                                                onvalue=counter - 1,
                                                                offvalue=-1,
                                                                variable=self.selected_cards[counter - 1][1]))
            self.selected_cards[counter - 1][4].grid(row=1, column=counter)
            counter += 1

    def show_card_table(self):
        global card_width
        global card_height
        for child in self.card_on_table.winfo_children():
            child.destroy()
        counter = 1
        for card_p in self.table:
            card_image = Label(self.card_on_table)
            card_image.grid(row=1, column=counter)
            counter += 1
            photo = ImageTk.PhotoImage(
                Image.open("graphics/cards/" + card_p.image + ".png").resize((card_width, card_height)))
            card_image.configure(image=photo)
            card_image.image = photo


class Example(Frame):
    def __init__(self, parent):
        Frame.__init__(self, parent)
        self.canvas = Canvas(self, borderwidth=0, background="#ffffff")
        self.frame = Frame(self.canvas, background="#ffffff")
        self.vsb = Scrollbar(self, orient="vertical", command=self.canvas.yview)
        self.canvas.configure(yscrollcommand=self.vsb.set)

        self.vsb.pack(side="right", fill="y")
        self.canvas.pack(side="left", fill="both", expand=True)
        self.canvas.create_window((4, 3), window=self.frame, anchor="nw",
                                  tags="self.frame")

        self.frame.bind("<Configure>", self.on_frame_configure)
        self.textArea = Label(self.frame)
        self.textArea.grid()

    def add_line_to_chat(self, text):
        self.textArea['text'] += text

    def on_frame_configure(self, event):
        self.canvas.configure(scrollregion=self.canvas.bbox("all"))


def open_login_window():
    def command_button(ip, window):
        gameClient.settings.save_adress(ip)
        if not gameClient.settings.name:
            open_name_window()
        response = gameClient.connect(ip)
        status_bar_text.set(response[1])
        if not response[0]:
            window.close()

    global gameClient
    status_bar_text = StringVar()
    login_window = Toplevel(master)
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


def open_name_window():
    def command_button(name):
        gameClient.settings.save_name(name)
        status_bar_text.set(gameClient.set_name(name))

    global gameClient
    status_bar_text = StringVar()
    name_window = Toplevel(master)
    name_window.title("Nick")
    Label(name_window, text="Enter nickname:").grid(row=1, column=1)
    address = Entry(name_window)
    address.insert(END, str(gameClient.settings.name))
    address.grid(row=1, column=2)
    name_button = Button(name_window, text="Change")
    name_button['command'] = lambda: command_button(address.get())
    name_button.grid(row=2, column=1, columnspan=2)
    status_bar = Label(name_window, textvariable=status_bar_text, relief='sunken')
    status_bar.grid(row=3, column=1, columnspan=2, sticky='we')


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


def send_message():
    gameClient.send({"chat": chatReadText.get()})


def send_report():
    gameClient.send({"report": ""})


def pass_button():
    gameClient.send({"pass": ""})


def play_button():
    play = []
    for card_block in client_holder.selected_cards:
        if card_block[1].get() >= 0:
            play.append(card_block[0].to_list())
    gameClient.send({"play": play})


card_height = 100
card_width = 70

master = Tk()
master.title("Squad of Five")
master.configure(bg='grey')
master.iconbitmap('graphics/icon.ico')

client_holder = ClientHolder()
gameClient = GameClient(client_holder)

# Adding menu
menu = Menu(master)
master.config(menu=menu)
subMenu = Menu(menu)
menu.add_cascade(label="Game", menu=subMenu)
subMenu.add_command(label="Connect", command=open_login_window)
subMenu.add_command(label="Change nickname", command=open_name_window)
subMenu.add_command(label="Host", command=host_game)

table = [Card(4, 1)]
# region=Main Zone
# region=Player Zone
player_zone = Label(master, bg="darkGrey")
player_zone.grid(row=1, column=1)

player1 = Label(player_zone, text="Player1")
player2 = Label(player_zone, text="Player2")
player3 = Label(player_zone, text="Player3")
cardCount1 = Label(player_zone, text=16)
cardCount2 = Label(player_zone, text=16)
cardCount3 = Label(player_zone, text=16)
client_holder.player_show_list = [[player1, cardCount1], [player2, cardCount2], [player3, cardCount3]]
player1.grid(column=1, row=2, sticky=E)
player2.grid(column=2, row=2)
player3.grid(column=3, row=2, sticky=E)
cardCount1.grid(column=1, row=3, sticky=E)
cardCount2.grid(column=2, row=3)
cardCount3.grid(column=3, row=3, sticky=E)

# endregion=Player Zone
# region=Card Zone
table_zone = Label(master)
client_holder.card_on_table = table_zone
table_zone.grid(row=2, column=1)

# endregion=Card Zone
# region=Deck Zone
deck_zone = Label(master, bg="lightGrey")
client_holder.cards_on_hand = deck_zone
deck_zone.grid(row=3, column=1)

# endregion=Deck Zone
# region=DeckButton Zone
deckButton_zone = Label(master, relief='sunken')
deckButton_zone.grid(row=4, column=1, sticky='we')

deckButton_zone_passButton = Button(deckButton_zone, text="Pass", command=pass_button)
deckButton_zone_passButton.grid(row=1, column=1)
deckButton_zone_playButton = Button(deckButton_zone, text="Play", command=play_button)
deckButton_zone_playButton.grid(row=1, column=2, sticky='we')

# endregion=DeckButton Zone
# region=Status Bar
statusBar = Label(master, text="Welcome", relief='sunken')
client_holder.status_bar = statusBar
statusBar.grid(row=5, column=1, columnspan=2, sticky='we')

# endregion=Status Bar
# region=Chat Zone

chatZone = Label(master)
example = Example(chatZone)
client_holder.add_line_to_chat = example.add_line_to_chat
example.pack(side="top", fill="both", expand=True)
chatZone.grid(column=2, row=1, rowspan=4)
chatInput = Label(chatZone)
chatReadText = Entry(chatInput)

chatSendButton = Button(chatInput, text="Send", command=send_message)
chatReportButton = Button(chatInput, text="Report", command=send_report)

chatInput.pack(side="bottom")
chatReadText.grid()
chatSendButton.grid()
chatReportButton.grid()

# endregion=Chat Zone
# endregion=Main Zone
mainloop()
