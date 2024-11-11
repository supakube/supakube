list:
    just --list

prepare-hot-reload:
    earthly -P +hot-reload
    k3d image import ghcr.io/supakube/hot-reload:latest
    kubectl patch deployment hot-reload -n supakube -p \
    "{\"spec\": {\"template\": {\"spec\": {\"containers\": [{\"name\": \"hot-reload\", \"image\": \"ghcr.io/supakube/hot-reload:latest\", \"imagePullPolicy\": \"Never\"}]}}}}"

deploy:
    cargo run --bin supakube -- install

chill:
    cargo watch --workdir /workspace/ \
        -w crates/web-server \
        "build --bin web-server"