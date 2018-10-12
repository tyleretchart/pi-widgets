import RPi.GPIO as gpio

gpio.setmode(gpio.BCM)
gpio.setup(26, gpio.IN) #, pull_up_down=gpio.PULL_DOWN)

while True:
    print(gpio.input(26))
