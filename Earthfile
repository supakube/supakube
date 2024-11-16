VERSION 0.8

FROM purtontech/rust-on-nails-devcontainer:1.3.3

ARG --global OPERATOR_IMAGE_NAME=supakube/supakube:latest
ARG --global OPERATOR_EXE_NAME=supakube

WORKDIR /build

USER vscode

all:
    BUILD +operator-container

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    # We need to run inside docker as we need postgres running for cornucopia
    RUN cargo build --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$OPERATOR_EXE_NAME

build-cli-linux:
    COPY --dir crates/k8s-operator .
    RUN cd k8s-operator && cargo build --release
    SAVE ARTIFACT supakube/target/release/supakube AS LOCAL ./supakube-cli-linux

# We've got a Kubernetes operator
operator-container:
    FROM scratch
    COPY +build/$OPERATOR_EXE_NAME k8s-operator
    ENTRYPOINT ["./k8s-operator", "operator"]
    SAVE IMAGE --push $OPERATOR_IMAGE_NAME