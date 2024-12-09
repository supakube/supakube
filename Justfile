list:
    just --list

init:
    k3d cluster delete
    k3d cluster create --agents 1 -p "30000-30001:30000-30001@agent:0"

deploy:
    cargo run --bin supakube -- install