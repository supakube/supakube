FROM mcr.microsoft.com/devcontainers/rust:1 as builder

WORKDIR /build

COPY ./ ./

RUN cargo build


FROM mcr.microsoft.com/devcontainers/rust:1 as runner


WORKDIR /app

COPY --from=builder /build/target/debug/web-server .

CMD ["./web-server"]