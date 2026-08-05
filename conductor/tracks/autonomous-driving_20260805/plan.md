# Implementation Plan: Autonomous Driving & Object Recognition

### Phase 1: YOLOv8 Inference Server on Desktop Backend
- [x] Task: Integrate YOLOv8 into Rust
    - [x] Import ONNX Runtime or a Rust-native YOLO library into the `src-tauri` backend.
    - [x] Download and load a pre-trained YOLOv8 model (e.g., `yolov8n.onnx`).
- [x] Task: Frame Interception & Inference
    - [x] Intercept the incoming MJPEG frame data before broadcasting to the UI.
    - [x] Run inference on frames (downscaled for performance) at a target 15+ FPS.
- [x] Task: Inference Broadcasting (Mobile Support)
    - [x] Extract bounding boxes and class labels from the model output.
    - [x] Broadcast these bounding boxes via the Tauri WebSocket/Event system to all connected UI clients.

### Phase 2: Frontend Visualization
- [x] Task: UI Bounding Box Overlay
    - [x] Create a Svelte component that parses incoming bounding box JSON.
    - [x] Overlay SVG or absolutely positioned HTML elements over the video feed to draw the bounding boxes and labels.
- [x] Task: Autonomous Controls UI
    - [x] Add an "Autonomous Mode" toggle switch to the UI.
    - [x] Add a dropdown to select the target class (e.g., "person", "bottle", "stop sign").

### Phase 3: Autonomous Logic Engine
- [x] Task: Safety Override
    - [x] Implement logic in the backend to immediately disengage Autonomous Mode if manual steering/throttle input is detected.
- [x] Task: Stop & Evasion Logic
    - [x] Calculate the center-point and area of bounding boxes.
    - [x] If an obstacle is detected directly in the center zone exceeding a critical size, send a `throttle: 0` command.
- [x] Task: Target Tracking Logic
    - [x] If tracking is active, calculate the horizontal offset of the target bounding box from the screen center.
    - [x] Map the offset to a steering value (PID or proportional control) and send steering commands to the Pi.

### Phase 4: Integration & Testing
- [x] Task: Desktop/Mobile Network Testing
    - [x] Verify that running the Desktop app acts as a local server, allowing the iOS app to view bounding boxes and toggle autonomous mode without running the ML model locally.
- [x] Task: Documentation Updates
    - [x] Log changes in `conductor/in-flight-changes.md`.
