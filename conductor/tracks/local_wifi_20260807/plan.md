# Implementation Plan: Local Wi-Fi AP Mode & Pairing

## Phase 1: Local Wi-Fi AP Mode (Network Manager)
- [ ] Task: Research and select a tool for dynamically switching the Pi's Wi-Fi to AP mode (e.g., `NetworkManager` nmcli or `autohotspot`).
- [ ] Task: Write a bash script to automatically configure the AP SSID (including MAC address) and WPA2 password on boot if standard connection fails.
- [ ] Task: Create or configure a systemd service that manages this fallback network logic.

## Phase 2: ADC Monitoring & Pairing Logic (Python Backend)
- [ ] Task: Implement a module in the Python backend to continuously read the Freenove battery voltage ADC channel.
- [ ] Task: Write state-machine logic to detect the distinct voltage drops and spikes representing a physical motor switch toggle.
- [ ] Task: Track the "is_paired" state based on detecting two consecutive toggles.
- [ ] Task: Create a connection gate in the WebSocket handler that refuses control inputs until "is_paired" becomes true.

## Phase 3: Integration & Testing
- [ ] Task: Document the new AP network requirements and manual pairing step in `README.md`.
- [ ] Task: Log the feature implementation in `conductor/in-flight-changes.md`.
- [ ] Task: End-to-end testing: Boot without known Wi-Fi -> connect to Pi AP -> attempt control (rejected) -> toggle motor switch twice -> control accepted.
