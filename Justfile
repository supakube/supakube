list:
    just --list

init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

# Deploy Postgres and Nginx operators
deploy-operators:
    cargo run --bin supakube -- install --no-supakube-operator

deploy-app:
    kubectl create namespace test-app
    kubectl apply -f .devcontainer/supakube.yaml
    cargo run --bin supakube -- open-ports --namespace test-app
    cargo run --bin supakube -- operator

watch:
    mold -run cargo watch --workdir /workspace/ -w crates/web-server -w crates/web-pages -w crates/db --no-gitignore -x "run --bin web-server"