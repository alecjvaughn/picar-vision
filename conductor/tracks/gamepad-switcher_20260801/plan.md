# Plan: Gamepad Switcher Modal

## Phase 1: Backend Logic (Rust)
- [ ] In `lib.rs`, add atomic state `ACTIVE_GAMEPAD` to store the active ID (using `AtomicUsize` where max is disconnected, since `GamepadId` is a `usize` under the hood in `gilrs`).
- [ ] Write `list_gamepads` Tauri command that locks the `gilrs` instance (or queries it if possible safely) and returns `Vec<{ id: usize, name: String }>`.
- [ ] Write `set_active_gamepad(id: Option<usize>)` command to update the `ACTIVE_GAMEPAD` state.
- [ ] In `controller.rs`, modify the `gilrs` loop to ONLY process events for `gamepad.id().into() == ACTIVE_GAMEPAD` (or if none is selected, maybe auto-select the first, or strictly require manual selection if none).

## Phase 2: Frontend Logic (Svelte)
- [ ] Add `showGamepadModal` state.
- [ ] Add "Find Gamepad" button next to the Controller Status.
- [ ] Create the HTML and CSS for the Modal.
- [ ] Implement `fetchGamepads()` calling `list_gamepads`.
- [ ] Implement UI buttons to "Set Active" or "Disconnect" gamepads.

## Phase 3: Finalization
- [ ] End-to-end testing (simulated).
- [ ] Update `in-flight-changes.md` and commit.
