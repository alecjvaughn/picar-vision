# Specification: iOS App Clone & Landscape Interface

## 1. Overview
Create an iOS clone of the existing Picar-Vision desktop application using Tauri Mobile. The iOS app will feature a new landscape-friendly orientation, displaying the live video feed prominently in the background with translucent virtual inputs overlaid on top. Less frequently used settings and telemetry will be organized into a collapsible menu.

## 2. Functional Requirements
*   **Tauri Mobile Integration:** Initialize and configure the iOS target within the existing Tauri project.
*   **Landscape Orientation:** Lock the iOS application to landscape orientation.
*   **Immersive Video Feed:** Display the live camera feed full-screen as the background layer.
*   **Toggleable Translucent Overlays:** 
    *   Create translucent telemetry overlay (Distance, Light).
    *   Create translucent virtual touchscreen joysticks (Left: Motor, Right: Camera).
    *   Add a global opacity setting slider in the Settings Sidebar that modifies the opacity of these elements.
    *   Add a toggle in the Settings Sidebar to hide/show the telemetry overlay independently.
*   **Smart Visibility:** Automatically hide virtual joysticks when an external physical Gamepad (e.g. Joy-Cons) is detected as connected. Re-show virtual joysticks when the gamepad disconnects.
*   **Collapsible UI Menu:** 
    *   Create a hamburger menu or translucent toggle button to open a sidebar/modal.
    *   Move the connection IP input, connection status, battery, opacity sliders, and Gamepad settings into this collapsible menu to reduce screen clutter.
*   **Deployment Guide:** Provide a Markdown document containing step-by-step instructions for the user to deploy the app to their personal iOS device using Xcode direct deployment (free provisioning profile).

## 3. Non-Functional Requirements
*   **Responsiveness:** The UI overlays must use CSS safe-area insets to avoid the iPhone notch and home indicator.
*   **Performance:** The video feed must maintain low latency with the translucent UI layered over it.

## 4. Acceptance Criteria
*   [ ] The Tauri project successfully builds an iOS Xcode project.
*   [ ] The application runs in landscape mode on an iOS device/simulator.
*   [ ] Virtual joysticks successfully control the motors and servos when tapped on the touchscreen.
*   [ ] The video feed is visible behind the translucent joysticks.
*   [ ] All other UI elements are cleanly organized inside a collapsible menu.
*   [ ] Virtual joysticks auto-hide when a physical gamepad is connected.
*   [ ] A deployment guide is available and accurate.

## 5. Out of Scope
*   Publishing to the Apple App Store.
