
# Build and load a service into docker
build-docker-rs SERVICE:
    cp Cargo.lock services/rs-{{SERVICE}}/
    nix build ./services/rs-{{SERVICE}}#dockerImage && ./result | docker load