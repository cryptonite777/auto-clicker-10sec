import time
import subprocess
from pynput.mouse import Controller, Button

mouse = Controller()

print("Process running,press ctrl+c to end")
while True:
    try:
        title = subprocess.check_output(["xdotool","getactivewindow","getwindowname"],text = True).strip()

        if "Minecraft" in title:
            mouse.click(Button.left)


    except subprocess.CalledProcessError:
        print("Maybe try sudo pacman -S xdotool")
        print("also make sure you get python-pip and do pip install pynput")

        pass

    time.sleep(10)
