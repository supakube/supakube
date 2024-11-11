VERSION 0.8

FROM purtontech/rust-on-nails-devcontainer:1.3.3

ARG --global HOT_RELOAD_IMAGE_NAME=ghcr.io/supakube/hot-reload:latest

WORKDIR /build

USER vscode

all:
    BUILD +hot-reload

hot-reload:
    FROM debian:12-slim
    # Set working directory
    WORKDIR /app
    COPY +build/hot-reload /app/web-server
    # Install necessary packages
    RUN apt-get update && \
        apt-get install -y --no-install-recommends inotify-tools && \
        rm -rf /var/lib/apt/lists/*
    # Create necessary directories
    RUN mkdir -p /app /var/log /workspace/crates/web-assets
    # Ensure the executable has execution permissions
    RUN chmod +x /app/web-server
    # Copy the update script into the container
    COPY crates/hot-reload/update_exec.sh /usr/local/bin/update_exec.sh
    # Ensure the script has execution permissions
    RUN chmod +x /usr/local/bin/update_exec.sh
    # Set the update script as the entry point
    ENTRYPOINT ["/usr/local/bin/update_exec.sh"]
    SAVE IMAGE --push $HOT_RELOAD_IMAGE_NAME

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    # We need to run inside docker as we need postgres running for cornucopia
    RUN cargo build --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/hot-reload