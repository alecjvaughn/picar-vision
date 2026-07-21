import sys
import os

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from ultrasonic import Ultrasonic
from photoresistor import Photoresistor

class UltrasonicPart:
    """
    Donkeycar-compatible part for the Freenove Ultrasonic sensor.
    Outputs: distance (float, in cm)
    """
    def __init__(self):
        self.sensor = Ultrasonic()
    
    def run(self):
        return self.sensor.get_distance()
        
    def shutdown(self):
        self.sensor.close()

class PhotoresistorPart:
    """
    Donkeycar-compatible part for the Freenove Photoresistors.
    Outputs: left_light (float), right_light (float)
    """
    def __init__(self):
        self.sensor = Photoresistor()

    def run(self):
        return self.sensor.read_left_photoresistor(), self.sensor.read_right_photoresistor()

    def shutdown(self):
        self.sensor.stop()
