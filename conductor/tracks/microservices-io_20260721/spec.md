# Track Specification: Microservices Deployment & Pi I/O Testing

## Overview
This track focuses on transitioning the legacy Freenove 4WD Python codebase into a modern microservices architecture running directly on the Raspberry Pi 3B+ (edge deployment). It establishes the foundational WebSocket communication layer to support low-latency remote control and live telemetry/video streaming to the future Tauri desktop app.

## Functional Requirements
- **Edge Deployment:** Containerize and deploy the core services directly to the Raspberry Pi 3B+.
- **Hardware I/O Abstraction:** Implement clean Python interfaces to test and control:
  - Motors (L298N/PCA9685 via I2C/GPIO)
  - Servos (Camera Pan/Tilt)
  - Camera (OpenCV ingestion, future-proofed for YOLOv8)
  - Sensors (Ultrasonic, Line Tracking, Photoresistor)
- **WebSocket Gateway:** Implement a low-latency WebSocket server on the Pi to receive steering/control commands and broadcast telemetry and MJPEG/H264 video frames.
- **Donkeycar Compatibility:** Ensure the I/O layer separates input, state, and output, allowing Donkeycar "parts" to easily slot into the control loop in future tracks.

## Non-Functional Requirements
- **Latency:** End-to-end control latency over local WiFi must be kept under 100ms.
- **Resource Efficiency:** The microservices must be lightweight to run reliably on the 1GB RAM of a Raspberry Pi 3B+.

## Acceptance Criteria
- [ ] Core services are successfully containerized (Docker) and running on the Pi.
- [ ] A test client can connect to the Pi via WebSockets.
- [ ] Motor commands sent via WebSockets successfully drive the wheels.
- [ ] Servo commands sent via WebSockets successfully pan/tilt the camera.
- [ ] Sensor data (e.g., ultrasonic distance) is successfully streamed back over WebSockets.
- [ ] The camera feed is successfully captured and streamed over the network.

## Out of Scope
- Building the actual Tauri desktop application (this track only builds the Pi backend).
- YOLOv8 AI inference (only the raw camera ingestion is tested here).
- Full GitOps/ArgoCD cloud deployment (focus is strictly on the edge Pi execution).
