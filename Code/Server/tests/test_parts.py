import unittest
from unittest.mock import MagicMock, patch
import sys
import os

sys.modules['smbus'] = MagicMock()
sys.modules['spidev'] = MagicMock()
sys.modules['RPi'] = MagicMock()
sys.modules['RPi.GPIO'] = MagicMock()
sys.modules['gpiozero'] = MagicMock()

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from parts.actuators import FreenoveMotorPart, FreenoveServoPart
from parts.sensors import UltrasonicPart, PhotoresistorPart

class TestActuators(unittest.TestCase):
    @patch('parts.actuators.Ordinary_Car')
    def test_motor_part(self, MockCar):
        motor_part = FreenoveMotorPart()
        mock_car_instance = motor_part.car
        
        # Test full forward
        motor_part.run(steering=0.0, throttle=1.0)
        mock_car_instance.set_motor_model.assert_called_with(4095, 4095, 4095, 4095)
        
        # Test full reverse
        motor_part.run(steering=0.0, throttle=-1.0)
        mock_car_instance.set_motor_model.assert_called_with(-4095, -4095, -4095, -4095)
        
        # Test full right turn in place
        motor_part.run(steering=1.0, throttle=0.0)
        mock_car_instance.set_motor_model.assert_called_with(4095, 4095, -4095, -4095)

        motor_part.shutdown()
        mock_car_instance.close.assert_called_once()

    @patch('parts.actuators.Servo')
    def test_servo_part(self, MockServo):
        servo_part = FreenoveServoPart()
        mock_servo_instance = servo_part.servo
        
        # Test center
        servo_part.run(pan=0.0, tilt=0.0)
        mock_servo_instance.set_servo_pwm.assert_any_call('0', 90)
        mock_servo_instance.set_servo_pwm.assert_any_call('1', 90)
        
        # Test max right/up
        servo_part.run(pan=1.0, tilt=1.0)
        mock_servo_instance.set_servo_pwm.assert_any_call('0', 180)
        mock_servo_instance.set_servo_pwm.assert_any_call('1', 180)

class TestSensors(unittest.TestCase):
    @patch('parts.sensors.Ultrasonic')
    def test_ultrasonic_part(self, MockUltrasonic):
        sensor_part = UltrasonicPart()
        mock_ultrasonic_instance = sensor_part.sensor
        mock_ultrasonic_instance.get_distance.return_value = 15.5
        
        dist = sensor_part.run()
        self.assertEqual(dist, 15.5)

    @patch('parts.sensors.Photoresistor')
    def test_photoresistor_part(self, MockPhoto):
        sensor_part = PhotoresistorPart()
        mock_photo_instance = sensor_part.sensor
        mock_photo_instance.read_left_photoresistor.return_value = 2.5
        mock_photo_instance.read_right_photoresistor.return_value = 1.1
        
        left, right = sensor_part.run()
        self.assertEqual(left, 2.5)
        self.assertEqual(right, 1.1)

if __name__ == '__main__':
    unittest.main()
