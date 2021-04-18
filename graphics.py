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


master = Tk()
master.geometry("1200x500")
master.title("Squad of Five")
master.configure(bg='grey')
menu=Menu(master)
master.config(menu=menu)
subMenu=Menu(menu)
menu.add_cascade(label="Game", menu=subMenu)
subMenu.add_command(label="Connect", command=open_login_window)
subMenu.add_command(label="Host")


playerZone = Label(master, bg="darkGrey")
cardZone= Label(master)
deckZone = Label(master, bg="lightGrey")
statusBar=Label(master, text="This is a status bar", relief='sunken')

player1 = Label(playerZone, text=players[0].name)
player2 = Label(playerZone, text=players[1].name)
player3 = Label(playerZone, text=players[2].name)
cardCount1 = Label(playerZone, text=len(players[0].hand))
cardCount2 = Label(playerZone, text=len(players[1].hand))
cardCount3 = Label(playerZone, text=len(players[2].hand))

playerZone.grid()
player1.grid(column=1, row=2, sticky=E)
player2.grid(column=2, row=2)
player3.grid(column=3, row=2, sticky=E)
cardCount1.grid(column=1, row=3, sticky=E)
cardCount2.grid(column=2, row=3)
cardCount3.grid(column=3, row=3, sticky=E)

cardZone.grid()

deckZone.grid()
cardsOnHand = []
cardsOnHandChkBtn=[]
for card in players[3].hand:
    cardsOnHand.append(Checkbutton(deckZone, text=str(card)))
x = 1
for cardLabel in cardsOnHand:
    cardLabel.grid(column=x,row=1)
    x += 1
deckZonePlay=Button(deckZone,text="   OK   ")
deckZonePlay.grid(columnspan=len(players[3].hand))

statusBar.grid(sticky='we')
# mainloop, runs infinitely
mainloop()
