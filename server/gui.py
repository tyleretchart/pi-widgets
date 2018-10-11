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

import argparse
from flask import Flask, abort, request

import requests
import threading

#
# ------------------------------------------
# global address object


class Address:
    def __init__(self):
        self.current_pi = "0.0.0.0"


address = Address()

#
# ------------------------------------------
# flask init

app = Flask(__name__)
app.secret_key = 'super secret key'

#
# ------------------------------------------
# kivy object init
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


#
# ------------------------------------------
# flask endpoint defined
@app.route('/code', methods=['POST'])
def code():
    if "address" not in request.form and "code" not in request.form:
        return 'Send the right parameters: "address" and "code"'
    else:
        address.current_pi = request.form["address"]
        code_input.text = request.form["code"]
    return ""


# TODO: Remove
@app.route('/hello', methods=['GET'])
def hello():
    code_input.text = "hello world\n"
    return code_input.text


#
# ------------------------------------------
# kivy functions defined
def send_code(instance):
    # send code to raspberry pi
    url = address.current_pi + ":5002/code"
    try:
        r = requests.post(url, data={"code": code_input.text})
        print(r.text)
    except Exception as e:
        print("Couldn't connect due to |{}|".format(e))


#
# ------------------------------------------
# kivy main app
class PaletteApp(App):
    def build(self):
        send_button.bind(on_press=send_code)

        top_layout.add_widget(code_input)
        button_layout.add_widget(send_button)
        top_layout.add_widget(button_layout)

        return top_layout


#
# ------------------------------------------
# main
if __name__ == '__main__':
    # run flask
    server = threading.Thread(target=app.run, args=["localhost", 5001])
    server.start()

    # run gui
    PaletteApp().run()