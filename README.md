# Rusty Graph Federation 🦀

Learning about GraphQL Federation with Rust and Nix

## The Setup

- Two services, a fake `profile` and `feed`, are deployed and each publish graphql subgraphs
- Given a k8s annotation, the `discovery` service will be able to publish a directory of subgraphs deployed in a namespace
- The `gateway` service uses this `discovery` service to publish a federated supergraph, and handles request routing to the services

## Getting Started

### Dev

The `default.nix` in the root describes the dev environment. Use `nix-shell` to enter.

If not using nix, expect a standard Rustup and Node environment. `yarn` is used.

### Building Containers with Nix

```sh
# Enter Nix Shell
nix-shell

# Use the just commands
just -l

# Build service
just build-docker <service subdirectory name>

# Build and load image into docker
just load-docker <service subdirectory name>

# Build and load image into minikube
just load-minikube <service subdirectory name>

# Build everything and deploy example Kubernetes cluster
minikube start
just deploy

```
