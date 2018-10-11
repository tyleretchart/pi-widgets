import kivy

from kivy.app import App
from kivy.uix.button import Button
from kivy.uix.textinput import TextInput
from kivy.uix.label import Label
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.floatlayout import FloatLayout
from kivy.uix.gridlayout import GridLayout
from kivy.uix.anchorlayout import AnchorLayout
from kivy.uix.scatterlayout import ScatterLayout
from kivy.uix.scatter import Scatter
from kivy.core.window import Window

import socket

#
# ------------------------------------------
# code object


class Code():
    def __init__(self):
        self.code = ""


code = Code()

#
# ------------------------------------------
# kivy functions defined


def send_code(instance):
    text_input = None
    for c in instance.parent.parent.children:
        if type(c) == type(TextInput()):
            text_input = c
    code.code = text_input.text
    App.get_running_app().stop()


def on_focus(instance, value):
    instance.text = code.code


#
# ------------------------------------------
# kivy main app


class GuiApp(App):
    def build(self):
        # build widgets
        code_input = TextInput(
            hint_text="No button selected...",
            multiline=True,
            cursor_blink=True,
            cursor=True)
        send_button = Button(text="Send", background_color=[2, 1, 1, 1])
        top_layout = GridLayout(
            cols=2, rows=1, row_force_default=True, row_default_height=600)
        button_layout = GridLayout(
            cols=1, rows=2, row_force_default=True, row_default_height=100)

        # bind functions
        send_button.bind(on_press=send_code)
        code_input.bind(focus=on_focus)

        top_layout.add_widget(code_input)
        button_layout.add_widget(send_button)
        top_layout.add_widget(button_layout)

        return top_layout


#
# ------------------------------------------
# main

if __name__ == '__main__':
    host = "127.0.0.1"
    port = 5011
    gui = GuiApp()

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind((host, port))
    while True:
        s.listen(1)
        conn, addr = s.accept()
        print("Connection opened:\n{}\n{}".format(conn, addr))

        try:
            data = conn.recv(1024)
            code.code = data.decode()
            print("Code recieved:", code.code)

        except socket.error:
            print("Error Occured.")
            break

        gui.run()
        
        final_code = str.encode(code.code)
        print("Code sent:", final_code)
        conn.sendall(str.encode(code.code))
        conn.close()
        print("Connection closed")
        print()