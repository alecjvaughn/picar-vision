resource "kind_cluster" "default" {
  name           = "picar-vision-cluster"
  wait_for_ready = true
}

resource "helm_release" "argocd" {
  name             = "argocd"
  repository       = "https://argoproj.github.io/argo-helm"
  chart            = "argo-cd"
  namespace        = "argocd"
  create_namespace = true
  version          = "5.51.6"

  set {
    name  = "server.extraArgs"
    value = "{--insecure}"
  }
}
