# Workflow Guide: Picar-Vision

## 1. Branching Strategy (GitHub Flow)
- **`main` Branch:** The single source of truth. Must always be deployable and stable.
- **Feature Branches:** Create a new branch off `main` for every new feature, bug fix, or refactor.
  - **Naming Convention:** `<type>/<issue-number-or-short-desc>` (e.g., `feat/yolov8-integration`, `fix/steering-latency`, `chore/gha-setup`).

## 2. Commit Messages (Conventional Commits)
- Commits must follow the Conventional Commits specification to enable automated changelogs and semantic versioning.
- **Format:** `<type>(<optional scope>): <description>`
- **Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`.
- **Example:** `feat(vision): integrate YOLOv8 for object detection`

## 3. Pull Request & GitOps Process
- **Small PRs:** Keep Pull Requests small and focused on a single track or phase to expedite reviews.
- **CI Checks:** All PRs must pass GitHub Actions pipelines before merging. This includes:
  - Rust formatting (`cargo fmt`), linting (`cargo clippy`), and tests.
  - Python tests for legacy Pi codebase changes.
  - UI linting (`eslint`, `prettier`).
  - Terraform validation (`terraform fmt -check`, `terraform validate`).
- **ArgoCD Sync:** Once merged to `main`, ArgoCD will automatically detect changes in the manifests/charts and synchronize the local `kind` cluster state.

## 4. Change Tracking
- **In-Flight Changes Log:** All changes (bugs, features, enhancements) requested by the user and implemented during active development tracks MUST be documented in `conductor/in-flight-changes.md`.
- **Formatting:** Group changes logically (e.g., UI, Backend, Logic), state the parent track number, and include the associated commit hashes.
