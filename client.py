from tkinter import *
from logics import *


class ClientHolder:
    chatTextArea: Label
    status_bar: Label


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
cardZone.grid(row=2, column=1)
# region=Card Zone

cardsOnTable = []
for card in table:
    cardsOnTable.append(Label(cardZone, text=str(card)))
x = 1
for cardLabel in cardsOnTable:
    cardLabel.grid(column=x, row=1)
    x += 1

# endregion=Card Zone


deckZone = Label(master, bg="lightGrey")
deckZone.grid(row=3, column=1)
# region=Deck Zone

cardsOnHand = []
cardsOnHandChkBtn = []

for card in gameClient.hand:
    cardsOnHand.append(Checkbutton(deckZone, text=str(card)))
x = 1
for cardLabel in cardsOnHand:
    cardLabel.grid(column=x, row=1)
    x += 1

deckZonePlay = Button(deckZone, text="   OK   ")
deckZonePlay.grid(columnspan=max(1, len(gameClient.hand)))
# endregion=Deck Zone


statusBar = Label(master, text="Welcome", relief='sunken')
client_holder.status_bar = statusBar
statusBar.grid(row=4, column=1, sticky='we')
# region=Status Bar
# endregion=Status Bar


chatZone = Label(master)
chatZone.grid(column=2, row=1, rowspan=4)
# region=Chat Zone

chatText = Label(chatZone, text="")
client_holder.chatTextArea = chatText
chatInput = Label(chatZone)
chatReadText = Entry(chatInput)

chatSendButton = Button(chatInput, text="Send", command=send_message)

chatText.grid()
chatInput.grid()
chatReadText.grid()
chatSendButton.grid()

# endregion=Chat Zone


# endregion=Main Zone


# mainloop, runs infinitely
mainloop()
