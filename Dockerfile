FROM rust as builder
ENV HOME=/home/root
WORKDIR  $HOME/rapi
ADD src src
ADD Cargo.lock .
ADD Cargo.toml .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/rapi/target \
    cargo build --release \
    && cp target/release/tp-wik-dps-tp01 ./app

FROM debian:latest
WORKDIR app
ENV PORT=3002
COPY --from=builder /home/root/rapi/app .
EXPOSE $PORT
ENTRYPOINT ["./app"]