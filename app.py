# example.py

import my_app_ui

def change_message():
    print("Python callback called")
    my_app_ui.update_message("Hello from Python!")

my_app_ui.run_app(change_message)
print("UI has been closed")
