list:
    just --list

prepare-hot-reload:
    earthly -P +hot-reload
    k3d image import ghcr.io/supakube/hot-reload:latest

chill:
    cargo watch --workdir /workspace/ \
        -w crates/web-server \
        "build --bin web-server"