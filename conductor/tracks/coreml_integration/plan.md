# Implementation Plan: CoreML Native iOS Integration

## Phase 1: Model Conversion
- [x] Task: Set up a Python environment with `ultralytics` and `coremltools`.
- [x] Task: Export the YOLOv8 Nano model to CoreML format (`yolov8n.mlpackage`).
- [x] Task: Integrate the `.mlpackage` into the Tauri iOS Xcode project bundle.

## Phase 2: Tauri Plugin Scaffolding
- [x] Task: Generate a new Tauri v2 plugin (e.g., `tauri-plugin-coreml`) using the Tauri CLI.
- [x] Task: Configure the plugin's `Package.swift` and iOS dependencies.
- [x] Task: Register the plugin in `Application/src-tauri/Cargo.toml` and `lib.rs`.

## Phase 3: Swift Vision Implementation
- [ ] Task: Write the Swift class implementation to load the `yolov8n` CoreML model using `VNCoreMLModel`.
- [ ] Task: Create an exposed Tauri command in Swift (`run_inference`) that accepts base64 image data or raw byte arrays.
- [ ] Task: Implement `VNCoreMLRequest` logic to process the image and extract bounding boxes.
- [ ] Task: Format the bounding boxes into a JSON-compatible structure and return it to the Rust/JS caller.

## Phase 4: Integration & Conditional Logic
- [ ] Task: Update the Svelte frontend and `websocket.rs` telemetry loop to conditionally route inference requests to the CoreML plugin when running on iOS.
- [ ] Task: Test the integration on the iOS simulator (fallback to CPU/GPU if ANE is unavailable in sim).
- [ ] Task: Update `conductor/in-flight-changes.md` to document the new architecture.
- [ ] Task: Commit the plugin and changes via `git`.
