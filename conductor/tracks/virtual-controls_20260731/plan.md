# Implementation Plan: Virtual Controls & Sensitivity Dials

## Phase 1: UI Scaffolding & State Management
- [x] Task: Scaffold Virtual Controls UI
    - [x] Create UI components for Virtual Joystick, WASD keys, D-Pad, and Snap button in the Side Panel.
    - [x] Add the Motor Speed and Servo Sensitivity range sliders.
- [x] Task: Svelte State Wiring
    - [x] Initialize reactive `$state` variables for the virtual keys, joystick position, and slider values.

## Phase 2: Keyboard & UI Event Mapping
- [x] Task: Keyboard Event Listeners
    - [x] Implement `keydown` and `keyup` listeners for physical WASD keys.
    - [x] Bind these events to visually light up the virtual WASD keys and update the virtual joystick position.
    - [x] Dispatch motor command events to the Rust backend based on WASD state, scaled by the Motor Speed dial.
- [x] Task: Dial State Syncing
    - [x] Sync the Motor Speed and Servo Sensitivity slider values with the Rust backend via Tauri IPC commands.

## Phase 3: Gamepad Servo Controls (Rust Backend)
- [x] Task: Extend Controller Event Polling
    - [x] Update `controller.rs` to map D-Pad inputs to servo pan/tilt logic.
    - [x] Update `controller.rs` to map the Right Shoulder button to the snap-to-center logic.
- [x] Task: Sensitivity and Command Generation
    - [x] Track servo positions and apply the Servo Sensitivity dial value to calculate continuous movement.
    - [x] Send new servo commands via the active WebSocket connection.
- [x] Task: Emit Controller States to UI
    - [x] Emit Tauri events when D-Pad or Right Shoulder buttons are pressed to light up the corresponding virtual keys in the Svelte UI.

## Phase 4: Integration & Validation
- [x] Task: WebSocket Payload Updates
    - [x] Ensure the JSON command payloads generated in Rust correctly bundle both motor (throttle/steering) and servo (pan/tilt) values.
- [x] Task: Code Quality & Linting
    - [x] Run `cargo clippy` and `cargo fmt` on the Rust backend.
    - [x] Run `npm run check` and formatting on the Svelte UI.
    - [x] Verify low latency and visual responsiveness of all virtual keys.
