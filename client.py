from tkinter import *
from gameLogics import *

# from networking import *

"""n = Client()
print(n.send("Hello"))
print(n.send("word"))"""

gameState = GameState()


def open_login_window():
    def commandButton(ip, window):
        gameState.settings.save_adress(ip)
        if not gameState.settings.name:
            open_name_window()
        statusBarText.set(gameState.connect(ip, window))

    global gameState
    statusBarText = StringVar()
    loginWindow = Toplevel(master)
    loginWindow.title("Connect")
    Label(loginWindow, text="Enter host adress:").grid(row=1, column=1)
    adress = Entry(loginWindow)
    adress.insert(END, str(gameState.settings.adress))
    adress.grid(row=1, column=2)
    loginButton = Button(loginWindow, text="Connect")
    loginButton['command'] = lambda: commandButton(adress.get(), loginWindow)
    loginButton.grid(row=2, column=1, columnspan=2)
    statusBar = Label(loginWindow, textvariable=statusBarText, relief='sunken')
    statusBar.grid(row=3, column=1, columnspan=2, sticky='we')


def open_name_window():
    def commandButton(name, window):
        gameState.settings.save_name(name)
        statusBarText.set(gameState.set_name(name, window))

    global gameState
    statusBarText = StringVar()
    nameWindow = Toplevel(master)
    nameWindow.title("Nick")
    Label(nameWindow, text="Enter nickname:").grid(row=1, column=1)
    adress = Entry(nameWindow)
    adress.insert(END, str(gameState.settings.name))
    adress.grid(row=1, column=2)
    nameButton = Button(nameWindow, text="Change")
    nameButton['command'] = lambda: commandButton(adress.get(), nameWindow)
    nameButton.grid(row=2, column=1, columnspan=2)
    statusBar = Label(nameWindow, textvariable=statusBarText, relief='sunken')
    statusBar.grid(row=3, column=1, columnspan=2, sticky='we')


master = Tk()
master.title("Squad of Five")
master.configure(bg='grey')

# Adding menu
menu = Menu(master)
master.config(menu=menu)
subMenu = Menu(menu)
menu.add_cascade(label="Game", menu=subMenu)
subMenu.add_command(label="Connect", command=open_login_window)
subMenu.add_command(label="Change nickname", command=open_name_window)

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


def showCards():
    global gameState
    for card in gameState.hand:
        cardsOnHand.append(Checkbutton(deckZone, text=str(card)))
    x = 1
    for cardLabel in cardsOnHand:
        cardLabel.grid(column=x, row=1)
        x += 1


deckZonePlay = Button(deckZone, text="   OK   ")
deckZonePlay.grid(columnspan=max(1, len(gameState.hand)))
# endregion=Deck Zone


statusBar = Label(master, text="This is a status bar", relief='sunken')
statusBar.grid(row=4, column=1, sticky='we')
# region=Status Bar
# endregion=Status Bar


chatZone = Label(master)
chatZone.grid(column=2, row=1, rowspan=4)
# region=Chat Zone

chatText = Label(chatZone, text="God: Test message\nGod: Test message2")
chatInput = Label(chatZone)
chatReadText = Entry(chatInput)
chatSendButton = Button(chatInput, text="Send")

chatText.grid()
chatInput.grid()
chatReadText.grid()
chatSendButton.grid()

# endregion=Chat Zone


# endregion=Main Zone


# mainloop, runs infinitely
mainloop()
