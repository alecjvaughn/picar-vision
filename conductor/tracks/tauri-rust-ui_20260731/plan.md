# Implementation Plan: Modern Desktop Interface (Tauri & Rust UI)

## Phase 1: Tauri Application Scaffolding
- [x] Task: Initialize Tauri Project
    - [x] Create a new Tauri project in the `/Application` directory using the Svelte + TypeScript template
    - [x] Configure `tauri.conf.json` for macOS build targets and necessary permissions
    - [x] Setup initial Svelte routing and layout structure
    - [x] Clean up boilerplate code and configure styling

## Phase 2: Rust Backend Integration
- [x] Task: Establish WebSocket Client
    - [x] Add `tokio` and `tungstenite` (or `reqwest` websockets) dependencies to `Cargo.toml`
    - [x] Implement an asynchronous WebSocket client in Rust to connect to the Raspberry Pi IP
- [x] Task: Tauri State & Event Management
    - [x] Create a shared Rust state for managing the connection status
    - [x] Implement a loop that parses incoming JSON telemetry and MJPEG frames
    - [x] Emit parsed telemetry and frame data as Tauri events to the Svelte frontend

## Phase 3: Dashboard UI Implementation (Svelte)
- [x] Task: Connection Manager Component
    - [x] Build a UI component to input the Pi's IP address and display connection status
    - [x] Implement Tauri IPC calls to trigger the Rust WebSocket connection
- [x] Task: Live Video Feed Component
    - [x] Build a video display component that listens to Tauri frame events
    - [x] Render the MJPEG stream smoothly onto an HTML5 Canvas or `<img>` tag
- [x] Task: Telemetry Dashboard Component
    - [x] Build visual indicators (bars/gauges) for ultrasonic distance and photoresistor light levels
    - [x] Display network latency/ping

## Phase 4: JoyCon Controller Support
- [x] Task: Rust Controller Integration
    - [x] Add the `gilrs` (Game Input Library for Rust) crate to `Cargo.toml`
    - [x] Implement a polling loop in Rust to detect and read JoyCon inputs
    - [x] Map JoyCon analog sticks/buttons to steering and throttle values
- [x] Task: Command Dispatching
    - [x] Translate normalized controller inputs into JSON command payloads
    - [x] Send command payloads over the active WebSocket connection to the Pi
    - [x] Emit controller status events to the Svelte UI for visual feedback (e.g., "JoyCon Connected")
