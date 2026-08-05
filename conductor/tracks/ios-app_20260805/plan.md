# Implementation Plan: iOS App Clone & Landscape Interface

### Phase 1: Tauri Mobile Configuration & Initialization
- [ ] Task: Initialize Tauri iOS project
    - [ ] Run `tauri ios init` to scaffold the Xcode project.
    - [ ] Configure `tauri.conf.json` for mobile targets and identifiers.
    - [ ] Update `Cargo.toml` if necessary for mobile support.
- [ ] Task: Lock Application to Landscape Mode
    - [ ] Configure Xcode project settings or Tauri configuration to strictly enforce landscape orientation on iOS devices.

### Phase 2: Touch Interface & Layout Refactoring
- [ ] Task: Implement Settings Sidebar/Modal
    - [ ] Extract connection settings (IP input, button, battery) and new opacity sliders into a separate collapsible side-menu.
- [ ] Task: Immersive Fullscreen Video Feed
    - [ ] Update CSS to render the MJPEG video stream full screen.
    - [ ] Add CSS safe-area insets (`env(safe-area-inset-left/right)`) to prevent UI from overlapping the iOS notch or home indicator.
- [ ] Task: Integrate Toggleable & Adjustable Translucent Overlays
    - [ ] Create translucent telemetry overlay (Distance, Light).
    - [ ] Create translucent virtual touchscreen joysticks (Left: Motor, Right: Camera).
    - [ ] Add a global opacity setting slider in the Settings Sidebar that modifies the opacity of these elements.
    - [ ] Add a toggle in the Settings Sidebar to hide/show the telemetry overlay independently.

### Phase 3: Hardware Input Routing & Smart Visibility
- [ ] Task: Route Virtual Joystick Data
    - [ ] Bind Left Virtual Joystick output to the existing `joystickX` and `joystickY` state variables.
    - [ ] Bind Right Virtual Joystick output to the existing camera pan/tilt commands.
- [ ] Task: Smart Auto-Hide Logic
    - [ ] Implement reactive logic: automatically hide virtual joysticks when an external physical Gamepad (e.g. Joy-Cons) is detected as connected.
    - [ ] Re-show virtual joysticks when the gamepad disconnects.
- [ ] Task: Fallback & Cleanup
    - [ ] Verify that existing physical keyboard and Web Gamepad logic is unharmed and continues to work on Desktop browsers.

### Phase 4: Documentation & Finalization
- [ ] Task: Create iOS Deployment Guide
    - [ ] Draft a `docs/ios_deployment.md` file outlining Xcode direct deployment steps.
- [ ] Task: Documentation Updates
    - [ ] Log changes in `conductor/in-flight-changes.md`.
