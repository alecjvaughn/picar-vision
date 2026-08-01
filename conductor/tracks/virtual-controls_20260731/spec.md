# Specification: Virtual Controls & Sensitivity Dials

## Overview
Implement an interactive virtual control scheme in the Tauri/Svelte dashboard. The UI will feature an on-screen virtual joystick, virtual WASD keys, a virtual D-Pad, and a "snap-to-center" button that visually light up and respond to physical inputs. Additionally, range sliders will be introduced to globally scale motor speed and adjust camera servo sensitivity, with the physical gamepad D-Pad mapped to control the servos.

## Functional Requirements
1. **Virtual UI Elements**: Add visual representations of a joystick, WASD keys, a D-Pad, and a Right Shoulder button in the dashboard side panel. These virtual keys must light up/highlight in real-time when the corresponding physical keyboard key or gamepad button is pressed.
2. **Keyboard Mapping (WASD)**: Map the physical WASD keys to drive the car (throttle and steering). Pressing these keys must update the virtual WASD keys, update the virtual joystick position, and send the calculated commands to the backend.
3. **Motor Speed Dial (Slider)**: Add a range slider that acts as a global multiplier for motor inputs. For example, if the slider is at 50%, pushing the joystick fully forward only sends a 0.5 throttle command to the car.
4. **Servo Control (D-Pad)**: Map the physical controller's D-Pad to control the Raspberry Pi camera servos (Pan and Tilt). Pressing the physical D-Pad must also light up the virtual D-Pad keys on the screen.
5. **Servo Sensitivity Dial (Slider)**: Add a range slider that dictates the speed/step-size of the servo movement.
6. **Continuous Servo Movement**: While a D-Pad direction is held down, the servos must pan/tilt continuously, with the movement speed determined by the Servo Sensitivity Dial.
7. **Snap to Center (Right Shoulder)**: Map the Right Shoulder button (R1) on the gamepad to instantly snap the servos back to their default (0, 0) center position. This action must also be represented by a virtual button in the UI that lights up when pressed.

## Non-Functional Requirements
- The new sliders and virtual keys must cleanly integrate into the existing side panel layout without breaking responsiveness.
- Keyboard input event listeners and gamepad polling loops must maintain low latency to ensure safe driving.

## Acceptance Criteria
- [ ] The user can drive the car using WASD keys.
- [ ] The virtual WASD, D-Pad, and Right Shoulder keys visually light up when the corresponding physical inputs are pressed.
- [ ] The user can pan and tilt the camera using the physical gamepad D-Pad.
- [ ] Holding the D-Pad moves the camera continuously.
- [ ] Pressing the Right Shoulder button snaps the camera servos back to the center (0, 0).
- [ ] The Motor Speed slider accurately scales the maximum speed of the car.
- [ ] The Servo Sensitivity slider accurately scales how fast the camera moves when the D-Pad is held.
