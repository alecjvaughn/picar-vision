# Implementation Plan: iOS App Clone & Landscape Interface

### Phase 1: Tauri Mobile Configuration & Initialization
- [x] Task: Initialize Tauri iOS project
    - [x] Run `tauri ios init` to scaffold the Xcode project.
    - [x] Configure `tauri.conf.json` for mobile targets and identifiers.
    - [x] Update `Cargo.toml` if necessary for mobile support.
- [x] Task: Lock Application to Landscape Mode
    - [x] Configure Xcode project settings or Tauri configuration to strictly enforce landscape orientation on iOS devices.

### Phase 2: Touch Interface & Layout Refactoring
- [x] Task: Implement Settings Sidebar/Modal
    - [x] Extract connection settings (IP input, button, battery) and new opacity sliders into a separate collapsible side-menu.
- [x] Task: Immersive Fullscreen Video Feed
    - [x] Update CSS to render the MJPEG video stream full screen.
    - [x] Add CSS safe-area insets (`env(safe-area-inset-left/right)`) to prevent UI from overlapping the iOS notch or home indicator.
- [x] Task: Integrate Toggleable & Adjustable Translucent Overlays
    - [x] Create translucent telemetry overlay (Distance, Light).
    - [x] Create translucent virtual touchscreen joysticks (Left: Motor, Right: Camera).
    - [x] Add a global opacity setting slider in the Settings Sidebar that modifies the opacity of these elements.
    - [x] Add a toggle in the Settings Sidebar to hide/show the telemetry overlay independently.

### Phase 3: Hardware Input Routing & Smart Visibility
- [x] Task: Route Virtual Joystick Data
    - [x] Bind Left Virtual Joystick output to the existing `joystickX` and `joystickY` state variables.
    - [x] Bind Right Virtual Joystick output to the existing camera pan/tilt commands.
- [x] Task: Smart Auto-Hide Logic
    - [x] Implement reactive logic: automatically hide virtual joysticks when an external physical Gamepad (e.g. Joy-Cons) is detected as connected.
    - [x] Re-show virtual joysticks when the gamepad disconnects.
- [x] Task: Fallback & Cleanup
    - [x] Verify that existing physical keyboard and Web Gamepad logic is unharmed and continues to work on Desktop browsers.

### Phase 4: Documentation & Finalization
- [ ] Task: Create iOS Deployment Guide
    - [ ] Draft a `docs/ios_deployment.md` file outlining Xcode direct deployment steps.
- [ ] Task: Documentation Updates
    - [ ] Log changes in `conductor/in-flight-changes.md`.
