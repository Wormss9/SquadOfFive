from tkinter import *
from logics import *
from pathlib import Path


class ClientHolder:
    chatTextArea: Label
    status_bar: Label
    chat = ""
    card_zone: Label
    cards_on_hand: Label
    hand = []
    table = []
    add_line_to_chat = ""

    def show_cards(self, location):
        print("location " + location)
        if location == "hand":
            label = self.cards_on_hand
            cards = self.hand
        elif location == "table":
            label = self.cards_on_hand
            cards = self.table
        else:
            print("show cards: ", location)
            return
        for child in label.winfo_children():
            child.destroy()
        counter = 1
        for card_p in cards:
            card_image = Label(label)
            card_image.grid(row=1, column=counter)
            counter += 1
            photo = PhotoImage(file=Path("graphics/cards/" + card_p.image))
            # .resize((450, 350), Image. ANTIALIAS)
            card_image.configure(image=photo)
            card_image.image = photo


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
    game_server = GameServer("Hosted server")
    start_new_thread(start_server, (game_server,))
    gameClient.connect("127.0.0.1")


def start_server(game_server):
    while True:
        game_server.network.accept_connection()


def send_message():
    gameClient.send({"chat": chatReadText.get()})


master = Tk()
master.title("Squad of Five")
master.configure(bg='grey')

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

playerZone = Label(master, bg="darkGrey")
playerZone.grid(row=1, column=1)
# region=Player Zone

player1 = Label(playerZone, text="Player1")
player2 = Label(playerZone, text="Player2")
player3 = Label(playerZone, text="Player3")
cardCount1 = Label(playerZone, text=16)
cardCount2 = Label(playerZone, text=16)
cardCount3 = Label(playerZone, text=16)

player1.grid(column=1, row=2, sticky=E)
player2.grid(column=2, row=2)
player3.grid(column=3, row=2, sticky=E)
cardCount1.grid(column=1, row=3, sticky=E)
cardCount2.grid(column=2, row=3)
cardCount3.grid(column=3, row=3, sticky=E)

# endregion=Player Zone


cardZone = Label(master)
client_holder.card_zone = cardZone
cardZone.grid(row=2, column=1)
# region=Card Zone


# endregion=Card Zone


deckZone = Label(master, bg="lightGrey")
client_holder.cards_on_hand = deckZone
deckZone.grid(row=3, column=1)
# region=Deck Zone

deckZonePlay = Button(deckZone, text="   OK   ")
deckZonePlay.grid(columnspan=max(1, len(gameClient.hand)))
# endregion=Deck Zone


statusBar = Label(master, text="Welcome", relief='sunken')
client_holder.status_bar = statusBar
statusBar.grid(row=4, column=1, sticky='we')
# region=Status Bar
# endregion=Status Bar

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


chatZone = Label(master)
example = Example(chatZone)
client_holder.add_line_to_chat = example.add_line_to_chat
example.pack(side="top", fill="both", expand=True)
chatZone.grid(column=2, row=1, rowspan=4)
chatInput = Label(chatZone)
chatReadText = Entry(chatInput)

chatSendButton = Button(chatInput, text="Send", command=send_message)

chatInput.pack(side="bottom")
chatReadText.grid()
chatSendButton.grid()
# region=Chat Zone

# endregion=Chat Zone


# endregion=Main Zone


# mainloop, runs infinitely
mainloop()
