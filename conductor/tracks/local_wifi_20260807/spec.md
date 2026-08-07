# Specification: Local Wi-Fi AP Mode & Pairing

## Overview
Implement an automatic fallback Local Wi-Fi (AP mode) feature on the Raspberry Pi gateway. This allows users to connect directly to the car without a pre-existing home network. Additionally, implement a manual pairing confirmation step that requires the user to toggle the Freenove base's physical Motor Power switch twice.

## Functional Requirements
- **Wi-Fi Fallback:** The Pi must attempt to connect to a known Wi-Fi network on boot. If no known networks are available, it must automatically configure and broadcast its own hotspot (AP mode).
- **SSID Configuration:** The AP SSID must uniquely identify the car by appending its MAC address or hostname (e.g., `Picar-Vision-[MAC]`).
- **Physical Pairing Mechanism:** Once a client connects to the AP, the software must require the user to perform a physical pairing action to unlock full telemetry/control access.
- **Hardware Toggle Detection:** The backend software must detect the pairing action by monitoring the Freenove HAT (likely via ADC battery/power readings) for two consecutive toggles of the physical Motor Power switch.

## Non-Functional Requirements
- **Security:** The AP network should be protected with a default WPA2 passphrase.
- **Resilience:** The network service should reliably handle transitioning between client and AP mode during boot without manual intervention.

## Acceptance Criteria
- [ ] Booting the Pi in an area without known Wi-Fi networks results in a unique `Picar-Vision-[MAC]` SSID being broadcasted.
- [ ] The user can successfully connect a laptop/phone to this AP network.
- [ ] The user is denied control until they manually toggle the physical Motor Power switch on the car twice.
- [ ] Toggling the switch twice successfully authenticates the session and enables full car control.

## Out of Scope
- Building a mobile app for network configuration.
- Simultaneous Dual-Mode Wi-Fi (acting as an AP and a Client at the exact same time).
