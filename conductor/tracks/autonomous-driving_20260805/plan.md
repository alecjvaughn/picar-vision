# Implementation Plan: Autonomous Driving & Object Recognition

### Phase 1: YOLOv8 Inference Server on Desktop Backend
- [ ] Task: Integrate YOLOv8 into Rust
    - [ ] Import ONNX Runtime or a Rust-native YOLO library into the `src-tauri` backend.
    - [ ] Download and load a pre-trained YOLOv8 model (e.g., `yolov8n.onnx`).
- [ ] Task: Frame Interception & Inference
    - [ ] Intercept the incoming MJPEG frame data before broadcasting to the UI.
    - [ ] Run inference on frames (downscaled for performance) at a target 15+ FPS.
- [ ] Task: Inference Broadcasting (Mobile Support)
    - [ ] Extract bounding boxes and class labels from the model output.
    - [ ] Broadcast these bounding boxes via the Tauri WebSocket/Event system to all connected UI clients.

### Phase 2: Frontend Visualization
- [ ] Task: UI Bounding Box Overlay
    - [ ] Create a Svelte component that parses incoming bounding box JSON.
    - [ ] Overlay SVG or absolutely positioned HTML elements over the video feed to draw the bounding boxes and labels.
- [ ] Task: Autonomous Controls UI
    - [ ] Add an "Autonomous Mode" toggle switch to the UI.
    - [ ] Add a dropdown to select the target class (e.g., "person", "bottle", "stop sign").

### Phase 3: Autonomous Logic Engine
- [ ] Task: Safety Override
    - [ ] Implement logic in the backend to immediately disengage Autonomous Mode if manual steering/throttle input is detected.
- [ ] Task: Stop & Evasion Logic
    - [ ] Calculate the center-point and area of bounding boxes.
    - [ ] If an obstacle is detected directly in the center zone exceeding a critical size, send a `throttle: 0` command.
- [ ] Task: Target Tracking Logic
    - [ ] If tracking is active, calculate the horizontal offset of the target bounding box from the screen center.
    - [ ] Map the offset to a steering value (PID or proportional control) and send steering commands to the Pi.

### Phase 4: Integration & Testing
- [ ] Task: Desktop/Mobile Network Testing
    - [ ] Verify that running the Desktop app acts as a local server, allowing the iOS app to view bounding boxes and toggle autonomous mode without running the ML model locally.
- [ ] Task: Documentation Updates
    - [ ] Log changes in `conductor/in-flight-changes.md`.
