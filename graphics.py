from tkinter import *
from main import *


def open_login_window():
    # Toplevel object which will
    # be treated as a new window
    newWindow = Toplevel(master)

    # sets the title of the
    # Toplevel widget
    newWindow.title("Connect")

    # sets the geometry of toplevel
    newWindow.geometry("200x200")

    # A Label widget to show in toplevel
    Label(newWindow,
          text="Select server or host").pack()


# Creating window
master = Tk()
master.title("Squad of Five")
master.configure(bg='grey')

# Adding menu
menu = Menu(master)
master.config(menu=menu)
subMenu = Menu(menu)
menu.add_cascade(label="Game", menu=subMenu)
subMenu.add_command(label="Connect", command=open_login_window)
subMenu.add_command(label="Host")

table = [Card(4, 1)]
# region=Main Zone

playerZone = Label(master, bg="darkGrey")
playerZone.grid(row=1, column=1)
# region=Player Zone

player1 = Label(playerZone, text=players[0].name)
player2 = Label(playerZone, text=players[1].name)
player3 = Label(playerZone, text=players[2].name)
cardCount1 = Label(playerZone, text=len(players[0].hand))
cardCount2 = Label(playerZone, text=len(players[1].hand))
cardCount3 = Label(playerZone, text=len(players[2].hand))

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
for card in players[3].hand:
    cardsOnHand.append(Checkbutton(deckZone, text=str(card)))
x = 1
for cardLabel in cardsOnHand:
    cardLabel.grid(column=x, row=1)
    x += 1
deckZonePlay = Button(deckZone, text="   OK   ")
deckZonePlay.grid(columnspan=len(players[3].hand))
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
