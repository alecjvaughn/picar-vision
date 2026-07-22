import asyncio
import websockets
import json
import base64
import cv2
import time
from parts.actuators import FreenoveMotorPart, FreenoveServoPart
from parts.sensors import UltrasonicPart, PhotoresistorPart
from parts.camera import OpenCVCameraPart

# Initialize parts
motor, servo, ultrasonic, photo, camera = None, None, None, None, None
print("Initializing Motor...")
try: motor = FreenoveMotorPart()
except Exception as e: print(f"Motor error: {e}")
print("Initializing Servo...")
try: servo = FreenoveServoPart()
except Exception as e: print(f"Servo error: {e}")
print("Initializing Ultrasonic...")
try: ultrasonic = UltrasonicPart()
except Exception as e: print(f"Ultrasonic error: {e}")
print("Initializing Photoresistor...")
try: photo = PhotoresistorPart()
except Exception as e: print(f"Photoresistor error: {e}")
print("Initializing Camera...")
try: camera = OpenCVCameraPart(width=320, height=240, framerate=15)
except Exception as e: print(f"Camera error: {e}")
print("All parts initialization attempted.")

async def telemetry_loop(websocket):
    """Continuously send telemetry (sensor data & camera frames) to client."""
    while True:
        try:
            # Gather sensors
            distance = ultrasonic.run() if ultrasonic else 0.0
            left_light, right_light = photo.run() if photo else (0.0, 0.0)
            
            # Gather frame and encode to JPEG
            frame = camera.run() if camera else None
            frame_b64 = ""
            if frame is not None:
                ret, buffer = cv2.imencode('.jpg', frame, [int(cv2.IMWRITE_JPEG_QUALITY), 60])
                if ret:
                    frame_b64 = base64.b64encode(buffer).decode('utf-8')

            telemetry_data = {
                "type": "telemetry",
                "distance": distance,
                "left_light": left_light,
                "right_light": right_light,
                "frame": frame_b64,
                "timestamp": time.time()
            }
            
            await websocket.send(json.dumps(telemetry_data))
            await asyncio.sleep(0.05) # 20 Hz telemetry
        except websockets.exceptions.ConnectionClosed:
            break
        except Exception as e:
            print(f"Telemetry error: {e}")
            await asyncio.sleep(0.5)

async def command_loop(websocket):
    """Receive commands from the client and route to actuators."""
    async for message in websocket:
        try:
            data = json.loads(message)
            if data.get("type") == "command":
                steering = data.get("steering", 0.0)
                throttle = data.get("throttle", 0.0)
                pan = data.get("pan", 0.0)
                tilt = data.get("tilt", 0.0)
                
                if motor: motor.run(steering, throttle)
                if servo: servo.run(pan, tilt)
        except Exception as e:
            print(f"Command error: {e}")

async def handler(websocket):
    print(f"Client connected: {websocket.remote_address}")
    
    # Run both loops concurrently
    telemetry_task = asyncio.create_task(telemetry_loop(websocket))
    command_task = asyncio.create_task(command_loop(websocket))
    
    done, pending = await asyncio.wait(
        [telemetry_task, command_task],
        return_when=asyncio.FIRST_COMPLETED,
    )
    for task in pending:
        task.cancel()
        
    print(f"Client disconnected: {websocket.remote_address}")

async def main():
    print("Starting WebSocket gateway on ws://0.0.0.0:8765")
    async with websockets.serve(handler, "0.0.0.0", 8765):
        await asyncio.Future()  # run forever

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass
    finally:
        if motor: motor.shutdown()
        if servo: servo.shutdown()
        if ultrasonic: ultrasonic.shutdown()
        if photo: photo.shutdown()
        if camera: camera.shutdown()
