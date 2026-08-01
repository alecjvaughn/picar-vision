# Implementation Plan: Modern Desktop Interface (Tauri & Rust UI)

## Phase 1: Tauri Application Scaffolding
- [x] Task: Initialize Tauri Project
    - [x] Create a new Tauri project in the `/Application` directory using the Svelte + TypeScript template
    - [x] Configure `tauri.conf.json` for macOS build targets and necessary permissions
    - [x] Setup initial Svelte routing and layout structure
    - [x] Clean up boilerplate code and configure styling

## Phase 2: Rust Backend Integration
- [ ] Task: Establish WebSocket Client
    - [ ] Add `tokio` and `tungstenite` (or `reqwest` websockets) dependencies to `Cargo.toml`
    - [ ] Implement an asynchronous WebSocket client in Rust to connect to the Raspberry Pi IP
- [ ] Task: Tauri State & Event Management
    - [ ] Create a shared Rust state for managing the connection status
    - [ ] Implement a loop that parses incoming JSON telemetry and MJPEG frames
    - [ ] Emit parsed telemetry and frame data as Tauri events to the Svelte frontend

## Phase 3: Dashboard UI Implementation (Svelte)
- [ ] Task: Connection Manager Component
    - [ ] Build a UI component to input the Pi's IP address and display connection status
    - [ ] Implement Tauri IPC calls to trigger the Rust WebSocket connection
- [ ] Task: Live Video Feed Component
    - [ ] Build a video display component that listens to Tauri frame events
    - [ ] Render the MJPEG stream smoothly onto an HTML5 Canvas or `<img>` tag
- [ ] Task: Telemetry Dashboard Component
    - [ ] Build visual indicators (bars/gauges) for ultrasonic distance and photoresistor light levels
    - [ ] Display network latency/ping

## Phase 4: JoyCon Controller Support
- [ ] Task: Rust Controller Integration
    - [ ] Add the `gilrs` (Game Input Library for Rust) crate to `Cargo.toml`
    - [ ] Implement a polling loop in Rust to detect and read JoyCon inputs
    - [ ] Map JoyCon analog sticks/buttons to steering and throttle values
- [ ] Task: Command Dispatching
    - [ ] Translate normalized controller inputs into JSON command payloads
    - [ ] Send command payloads over the active WebSocket connection to the Pi
    - [ ] Emit controller status events to the Svelte UI for visual feedback (e.g., "JoyCon Connected")
