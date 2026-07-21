# Tech Stack: Picar-Vision

## Legacy / Base Code (Raspberry Pi & Existing Client)
- **Language:** Python
- **Framework:** OpenCV (legacy Haar Cascades), raw TCP sockets.
- **Hardware:** Raspberry Pi 3B+, Freenove 4WD Smart Car Kit.

## Modern Application Stack (Desktop & Cloud/Edge)
- **Frontend / UI:** Tauri (Desktop Application Framework)
- **Backend (Desktop App):** Rust (Performance-critical logic, hardware communication, video stream processing)
- **Computer Vision:** YOLOv8 (Real-time object detection and tracking)
- **Web Technologies:** HTML/CSS/JavaScript (for the Tauri UI webview)

## DevOps & Infrastructure (GitOps)
- **Local Cluster:** kind (Kubernetes IN Docker)
- **Infrastructure as Code:** Terraform
- **Continuous Deployment:** ArgoCD
- **Continuous Integration:** GitHub Actions (GHA) for automated builds, testing, and linting.

## Future Extensibility
- **Autonomous Driving Framework:** Donkeycar (planned compatibility for behavioral cloning and autonomous navigation)
