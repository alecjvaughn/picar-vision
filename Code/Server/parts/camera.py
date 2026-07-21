import cv2
import threading
import time

class OpenCVCameraPart:
    """
    Donkeycar-compatible camera part using OpenCV for continuous frame capture.
    Outputs: frame (numpy array)
    """
    def __init__(self, device_id=0, width=640, height=480, framerate=30):
        self.device_id = device_id
        self.width = width
        self.height = height
        self.framerate = framerate
        
        self.cap = cv2.VideoCapture(self.device_id)
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
            ret, frame = self.cap.read()
            if ret:
                self.frame = frame
            else:
                time.sleep(0.01)

    def run(self):
        return self.frame

    def shutdown(self):
        self.running = False
        self.thread.join(timeout=1.0)
        self.cap.release()
