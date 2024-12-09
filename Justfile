list:
    just --list

init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

deploy-operator:
    cargo run --bin supakube -- install

deploy-app:
    kubectl create namespace test-app
    kubectl apply -f .devcontainer/supakube.yaml