# Rusty Graph Federation 🦀

Learning about GraphQL Federation with Rust and Nix

## Long Term Plan

- Two services, a fake `profile` and `feed`, are deployed and each publish graphql subgraphs
- Given a k8s annotation, the `discovery` service will be able to publish a directory of subgraphs deployed in a namespace
- The `gateway` service uses this `discovery` service to publish a federated super graph, and handles ingresses to the services

## Getting Started

### Dev

The `default.nix` in the root describes the dev environment. Use `nix-shell` to enter.

If not using nix, expect a standard Rustup and Node environment. `yarn` is used.

### Building Containers with Nix

```sh
# Use the just commands
just -l

# Build rust servcies
just build-docker-rs service

# Build non-rust services
just build-docker gateway

```
