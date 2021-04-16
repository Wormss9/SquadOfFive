from tkinter import *

master = Tk()

master.geometry("500x500")
master.title("Squad of Five")

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

btn = Button(master,
             text="Play",
             command=open_login_window)
btn.pack(pady=10)


# mainloop, runs infinitely
mainloop()