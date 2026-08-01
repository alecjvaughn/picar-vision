# Implementation Plan: Robust Infrastructure (GitOps, Terraform, ArgoCD)

## Phase 1: Terraform Infrastructure as Code
- [ ] Task: Terraform Scaffolding
    - [ ] Create an `infrastructure/` directory at the project root
    - [ ] Initialize standard Terraform files (`main.tf`, `variables.tf`, `providers.tf`)
- [ ] Task: Local Kubernetes Provisioning
    - [ ] Add the `tehcyx/kind` Terraform provider
    - [ ] Define the `kind_cluster` resource to spin up a local Kubernetes cluster on Mac

## Phase 2: GitOps CD with ArgoCD
- [ ] Task: ArgoCD Installation
    - [ ] Use the `hashicorp/helm` provider in Terraform to install ArgoCD onto the `kind` cluster
    - [ ] Output the instructions/commands to retrieve the initial ArgoCD admin password and port-forward the UI
- [ ] Task: Root Application Setup
    - [ ] Create a foundational ArgoCD `Application` manifest (`infrastructure/argocd/root-app.yaml`) linking back to the `picar-vision` git repository using the App of Apps pattern

## Phase 3: Application Scaffolding in Kubernetes
- [ ] Task: Kubernetes Manifests for UI
    - [ ] Create a `deploy/ui/` directory with basic Kubernetes Deployment and Service manifests to serve the built Tauri/Svelte web assets via an NGINX container
- [ ] Task: ArgoCD Sync
    - [ ] Register the `deploy/ui/` manifests as an ArgoCD Application in the cluster
    - [ ] Validate that ArgoCD automatically syncs and deploys the UI pods within the `kind` cluster
