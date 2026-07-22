import sys
import os

# Add parent directory to path so we can import legacy modules
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from motor import Ordinary_Car
from servo import Servo

class FreenoveMotorPart:
    """
    Donkeycar-compatible part for the Freenove 4WD motors.
    Inputs: steering (float, -1.0 to 1.0), throttle (float, -1.0 to 1.0)
    """
    def __init__(self):
        self.car = Ordinary_Car()

    def run(self, steering, throttle):
        # Differential drive steering logic:
        left = throttle + steering
        right = throttle - steering
        
        # Clamp to -1.0..1.0
        left = max(-1.0, min(1.0, left))
        right = max(-1.0, min(1.0, right))

        duty_l = int(left * 4095)
        duty_r = int(right * 4095)

        self.car.set_motor_model(duty_l, duty_l, duty_r, duty_r)

    def shutdown(self):
        self.car.close()

class FreenoveServoPart:
    """
    Donkeycar-compatible part for the Freenove camera pan/tilt servos.
    Inputs: pan (float, -1.0 to 1.0), tilt (float, -1.0 to 1.0)
    """
    def __init__(self):
        self.servo = Servo()

    def run(self, pan, tilt):
        # Map -1.0..1.0 to 0..180 degrees (90 is center)
        pan_angle = int((pan + 1.0) * 90)
        tilt_angle = int((tilt + 1.0) * 90)

        pan_angle = max(0, min(180, pan_angle))
        tilt_angle = max(0, min(180, tilt_angle))

        # Channel 0 is Pan, Channel 1 is Tilt
        self.servo.set_servo_pwm('0', pan_angle)
        self.servo.set_servo_pwm('1', tilt_angle)

    def shutdown(self):
        pass

class FreenoveBuzzerPart:
    """
    Donkeycar-compatible part for the Freenove buzzer.
    Inputs: state (bool)
    """
    def __init__(self):
        from buzzer import Buzzer
        self.buzzer = Buzzer()

    def run(self, state):
        self.buzzer.set_state(bool(state))

    def shutdown(self):
        self.buzzer.close()

class FreenoveLedPart:
    """
    Donkeycar-compatible part for the Freenove LEDs.
    Inputs: mode (string) e.g., 'blink', 'rainbow', 'off'
    """
    def __init__(self):
        from led import Led
        self.led = Led()

    def run(self, mode):
        if mode == 'blink':
            self.led.colorBlink(1, 100)
        elif mode == 'rainbow':
            self.led.rainbowCycle(20)
        else:
            self.led.colorBlink(0)

    def shutdown(self):
        self.led.colorBlink(0)
