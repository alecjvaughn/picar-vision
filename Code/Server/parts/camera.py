import cv2
import threading
import time

class OpenCVCameraPart:
    """
    Donkeycar-compatible camera part using Picamera2 (for Pi) or OpenCV (fallback)
    Outputs: frame (numpy array)
    """
    def __init__(self, device_id=0, width=640, height=480, framerate=30):
        self.width = width
        self.height = height
        self.framerate = framerate
        
        self.picam2 = None
        self.cap = None
        
        try:
            from picamera2 import Picamera2
            self.picam2 = Picamera2()
            config = self.picam2.create_video_configuration(main={"size": (self.width, self.height), "format": "RGB888"})
            self.picam2.configure(config)
            self.picam2.start()
            print("✅ Picamera2 initialized successfully.")
        except Exception as e:
            print(f"Picamera2 failed: {e}. Falling back to OpenCV VideoCapture({device_id})")
            self.cap = cv2.VideoCapture(device_id)
            self.cap.set(cv2.CAP_PROP_FRAME_WIDTH, self.width)
            self.cap.set(cv2.CAP_PROP_FRAME_HEIGHT, self.height)
            self.cap.set(cv2.CAP_PROP_FPS, self.framerate)
        
        self.frame = None
        self.running = True
        
        self.thread = threading.Thread(target=self.update, args=())
        self.thread.daemon = True
        self.thread.start()

    def update(self):
        while self.running:
            if self.picam2:
                try:
                    frame = self.picam2.capture_array()
                    # Convert RGB (picamera2) to BGR (opencv)
                    self.frame = cv2.cvtColor(frame, cv2.COLOR_RGB2BGR)
                except Exception as e:
                    print(f"Picamera2 capture error: {e}")
                    time.sleep(0.01)
            elif self.cap:
                ret, frame = self.cap.read()
                if ret:
                    self.frame = frame
                else:
                    time.sleep(0.01)
            else:
                time.sleep(1)

    def run(self):
        return self.frame

    def shutdown(self):
        self.running = False
        self.thread.join(timeout=1.0)
        if self.picam2:
            self.picam2.stop()
            self.picam2.close()
        if self.cap:
            self.cap.release()
