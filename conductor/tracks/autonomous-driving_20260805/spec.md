# Specification: Autonomous Driving & Object Recognition

## 1. Overview
Implement an object recognition pipeline and a basic autonomous driving mode for the Picar-Vision system. The system will rely on YOLOv8 running on the Desktop backend (Rust) to process the live MJPEG feed, determine bounding boxes, and send reactive steering/throttle commands to the Raspberry Pi over WebSockets. The mobile iOS client will connect to the Desktop server to receive lightweight inference data (bounding boxes) for UI visualization without running heavy models locally.

## 2. Functional Requirements
*   **Computer Vision Engine:** Integrate YOLOv8 inference into the Rust Tauri backend.
*   **Video Processing Loop:** Intercept the incoming MJPEG stream, process frames through YOLOv8, and calculate bounding boxes for detected objects.
*   **Inference Broadcasting:** Broadcast bounding boxes and class labels over WebSockets/Events so external UI clients (e.g., iOS App) can render them without processing video locally.
*   **UI Overlay:** Transmit bounding box coordinates to the Svelte frontend and draw them directly over the live video feed.
*   **Autonomous Logic Loop:** Create a configurable control loop with the following reactive behaviors:
    *   **Obstacle Detection:** Stop the car if a large obstacle is detected directly in the forward path (central bounding box exceeding a size threshold).
    *   **Target Tracking:** Follow specific target objects (e.g., a person) by adjusting steering to keep the object centered, and adjusting throttle based on the object's bounding box size (distance estimation).
    *   **Evasion:** Steer away from detected obstacles when not explicitly in a "tracking" mode.
*   **UI Controls:** Add a toggle in the UI to enable/disable "Autonomous Mode" and select the primary tracking target class.

## 3. Non-Functional Requirements
*   **Latency:** The object recognition loop must execute fast enough to prevent the car from crashing before reacting (aiming for >15 FPS inference).
*   **Safety:** The manual joysticks/gamepad must immediately override or disable Autonomous Mode upon user input.

## 4. Acceptance Criteria
*   [ ] YOLOv8 model successfully loads and processes frames in the Rust backend.
*   [ ] Bounding boxes are accurately drawn on the Svelte UI video feed on both desktop and mobile clients.
*   [ ] The car stops when an obstacle is placed directly in front of it.
*   [ ] The car can successfully center a moving target (e.g., a person) in its field of view by steering.
*   [ ] Manual input instantly disables Autonomous Mode.

## 5. Out of Scope
*   Full Donkeycar behavioral cloning (reserved for future scalability).
*   Edge device (Raspberry Pi) inference.
