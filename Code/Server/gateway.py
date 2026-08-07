import asyncio
import websockets
import json
import base64
import cv2
import time
from parts.actuators import FreenoveMotorPart, FreenoveServoPart, FreenoveBuzzerPart, FreenoveLedPart
from parts.sensors import UltrasonicPart, PhotoresistorPart
from parts.camera import OpenCVCameraPart

is_paired = False

async def pairing_monitor():
    global is_paired
    try:
        from adc import ADC
        adc = ADC()
    except Exception as e:
        print(f"ADC init error: {e}")
        # If no ADC, default to paired to avoid bricking
        is_paired = True 
        return

    toggles = 0
    last_state = True # Assume ON
    last_toggle_time = time.time()
    
    while not is_paired:
        try:
            power = adc.read_adc(2) * (3 if adc.pcb_version == 1 else 2)
            current_state = power > 5.0 # Power switch is ON
            
            if current_state != last_state:
                if not current_state: # Flipped OFF
                    toggles += 1
                    last_toggle_time = time.time()
                last_state = current_state
                
            if toggles >= 2:
                print("Pairing successful! Motor switch toggled twice.")
                is_paired = True
                break
                
            if toggles > 0 and (time.time() - last_toggle_time) > 10.0:
                toggles = 0
                
            await asyncio.sleep(0.1)
        except Exception as e:
            print(f"Pairing monitor error: {e}")
            await asyncio.sleep(1.0)

# Initialize parts
motor, servo, buzzer, led, ultrasonic, photo, camera = None, None, None, None, None, None, None
print("Initializing Motor...")
try: motor = FreenoveMotorPart()
except Exception as e: print(f"Motor error: {e}")
print("Initializing Servo...")
try: servo = FreenoveServoPart()
except Exception as e: print(f"Servo error: {e}")
print("Initializing Buzzer...")
try: buzzer = FreenoveBuzzerPart()
except Exception as e: print(f"Buzzer error: {e}")
print("Initializing LED...")
try: led = FreenoveLedPart()
except Exception as e: print(f"LED error: {e}")
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

current_pan = 0.0
current_tilt = 0.0

current_led_mode = "off"

async def telemetry_loop(websocket):
    """Continuously send telemetry (sensor data & camera frames) to client."""
    global current_led_mode
    while True:
        try:
            # Update continuous tasks
            if led: led.run(current_led_mode)

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
                "is_paired": is_paired,
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
    global current_led_mode
    async for message in websocket:
        try:
            data = json.loads(message)
            if data.get("type") == "bounding_boxes":
                for client in connected_clients:
                    if client != websocket:
                        asyncio.create_task(client.send(message))

            if data.get("type") == "command":
                if not is_paired:
                    continue # Ignore all commands until pairing is complete
                
                steering = data.get("steering", 0.0)
                throttle = data.get("throttle", 0.0)
                
                global current_pan, current_tilt
                if "pan" in data:
                    current_pan = data["pan"]
                if "tilt" in data:
                    current_tilt = data["tilt"]
                    
                buzzer_state = data.get("buzzer", False)
                if "led" in data:
                    current_led_mode = data.get("led", "off")
                
                if motor: motor.run(steering, throttle)
                if servo: servo.run(current_pan, current_tilt)
                if buzzer: buzzer.run(buzzer_state)
        except Exception as e:
            print(f"Command error: {e}")

connected_clients = set()

async def handler(websocket):
    print(f"Client connected: {websocket.remote_address}")
    connected_clients.add(websocket)
    
    # Run both loops concurrently
    telemetry_task = asyncio.create_task(telemetry_loop(websocket))
    command_task = asyncio.create_task(command_loop(websocket))
    
    try:
        done, pending = await asyncio.wait(
            [telemetry_task, command_task],
            return_when=asyncio.FIRST_COMPLETED,
        )
        for task in pending:
            task.cancel()
    finally:
        connected_clients.remove(websocket)
        print(f"Client disconnected: {websocket.remote_address}")
        
        # Safety: Stop motors on disconnect (only if no clients left?)
        # For safety, let's stop motors if NO clients are connected
        if len(connected_clients) == 0 and motor:
            motor.run(0.0, 0.0)

async def main():
    print("Starting WebSocket gateway on ws://0.0.0.0:8765")
    asyncio.create_task(pairing_monitor())
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
        if buzzer: buzzer.shutdown()
        if led: led.shutdown()
        if ultrasonic: ultrasonic.shutdown()
        if photo: photo.shutdown()
        if camera: camera.shutdown()
