list:
    just --list

deploy-hot-reload:
    earthly -P +hot-reload
    k3d image import ghcr.io/supakube/hot-reload:latest
    kubectl patch deployment hot-reload -n supakube -p \
    "{\"spec\": {\"template\": {\"spec\": {\"containers\": [{\"name\": \"hot-reload\", \"image\": \"ghcr.io/supakube/hot-reload:latest\", \"imagePullPolicy\": \"Never\"}]}}}}"

install-supakube:
    cargo run --bin supakube -- install --development

chill:
    cargo watch -w crates/web-server -x "build --bin web-server"