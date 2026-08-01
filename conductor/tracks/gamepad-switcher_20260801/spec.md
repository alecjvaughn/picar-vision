# Specification: Gamepad Switcher Modal

## 1. Description
A feature allowing users to view all connected gamepads detected by `gilrs`, select an active gamepad, or "disconnect" (ignore) a gamepad in the UI without unpairing it at the OS level.

## 2. Requirements
- Add a "Find" controllers button to the UI (near the gamepad status).
- The button opens a modal listing all gamepads currently connected via `gilrs`.
- Users can click on a gamepad in the list to make it the active controller.
- Selecting a gamepad should override the default behavior where `gilrs` just takes the first or most recently active gamepad.
- Users can click "Disconnect" in the modal to ignore gamepads.

## 3. Architecture
- **Rust Backend:**
  - Add state to track `active_gamepad_id: Option<GamepadId>`.
  - Add Tauri command `list_gamepads` that queries `gilrs` for all connected pads.
  - Add Tauri command `set_active_gamepad(id)` to forcefully select a specific gamepad or `None` to disconnect all.
- **Svelte Frontend:**
  - `showGamepadModal` boolean state.
  - Periodic or on-open query to `list_gamepads`.
  - Modal UI to display gamepads (ID, Name, Power).
  - Selection logic calling `set_active_gamepad`.
