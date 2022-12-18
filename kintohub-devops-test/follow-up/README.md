# Deploy Node.js Example

This document will keep track of what I learnt through doing this task.

## Installation

Luckily, installing `kubectl`, `minikube` and `argo` are relatively simple on macOS.

```bash
$ brew install kubernetes-cli
$ brew cask install minikube
$ brew install argoproj/tap/argo
```

## Research

There are a lot of terms within the [Getting Started](https://argoproj.github.io/docs/argo/demo.html) guide for Argo that I don't know, so I did the following research.

### Namespace

In Kubernetes, namespaces provide a scope for names. For example, in Argo, all namespaced resources created by Argo would be created under that namespace, since Argo manages its own resources. That way, Argo's resources would be cleanly separated from the other resources created by the user.

### ClusterRole

Represents the "role" in RBAC. ClusterRole is similar to Role, but isn't limited to a namespace. It grants permissions to the users and accounts it is binded to, through the RoleBinding resource.

### Workflow

An Argo workflow is just a YAML file detailing a series of steps to run.

### Sidecar

"Sidecar" containers are present when a single Pod contains more than one container. These "sidecar" containers often perform tasks that are secondary to the main container within the Pod.

## Instructions

To run this completed test, the secrets for logging into Docker must first be created.

```bash
$ kubectl create secret generic docker-username --from-literal=docker-username=<DOCKER_USERNAME>
$ kubectl create secret generic docker-password --from-literal=docker-password=<DOCKER_PASSWORD>
```

Next, simply run the following.

```bash
$ argo submit --watch build-and-deploy.yaml
```
