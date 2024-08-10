
build-docker SERVICE_PATH:
    nix build ./services/{{SERVICE_PATH}}#dockerImage && ./result | docker load

# Build and load a service into docker
# TODO, figure out how to reference the top-level cargo lock via flake
build-docker-rs SERVICE:
    cp Cargo.lock services/rs-{{SERVICE}}/
    git add -N services/rs-{{SERVICE}}/Cargo.lock
    nix build ./services/rs-{{SERVICE}}#dockerImage && ./result | docker load
    rm services/rs-{{SERVICE}}/Cargo.lock
    