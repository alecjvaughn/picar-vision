# Specification: Robust Infrastructure (GitOps, Terraform, ArgoCD)

## Overview
Establish a robust, local GitOps-driven infrastructure for the Picar-Vision project. This track focuses on setting up a local Kubernetes environment (`kind`), defining infrastructure as code (Terraform), and implementing continuous deployment (ArgoCD) to manage the application lifecycle, specifically scaffolding the deployment pipelines for the Tauri/Svelte UI.

## Functional Requirements
- **Local Kubernetes Cluster:**
  - Provision a local Kubernetes cluster using `kind` (Kubernetes in Docker) on the development machine (Mac).
- **Infrastructure as Code (Terraform):**
  - Define the `kind` cluster and ArgoCD installation using Terraform.
  - Utilize a local `terraform.tfstate` file for simplicity in this initial phase.
- **Continuous Deployment (ArgoCD):**
  - Install and configure ArgoCD within the `kind` cluster.
  - Setup an ArgoCD Application to monitor the Picar-Vision Git repository.
- **Application Deployment Scaffolding:**
  - Create Kubernetes manifests or a Helm chart for the Tauri/Svelte UI web assets to allow serving them from the local cluster for browser-based testing and GitOps validation.

## Non-Functional Requirements
- **Reproducibility:** The entire infrastructure must be reproducible from scratch via Terraform.
- **GitOps Principles:** ArgoCD must automatically detect and apply changes committed to the repository.

## Acceptance Criteria
- `terraform apply` successfully creates a `kind` cluster and installs ArgoCD.
- User can access the ArgoCD web UI locally.
- ArgoCD successfully syncs the Tauri UI deployment from the Git repository into the `kind` cluster.

## Out of Scope
- Deploying the cluster to a remote cloud provider (e.g., AWS, GCP).
- Complex monitoring stacks (Prometheus/Grafana).
- Remote Terraform state management.
