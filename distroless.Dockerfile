ARG GLOBAL_ZIG_VER="zig-linux-aarch64-0.11.0"
ARG GLOBAL_TARGET="x86_64-unknown-linux-musl"
ARG GLOBAL_USER="testuser"

FROM rust:1-bookworm as base

RUN update-ca-certificates

RUN rustup target add x86_64-unknown-linux-musl \
    && apt-get update -y \
    && apt-get install -y musl-tools \
    && apt-get clean


WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

FROM base as builder

ARG GLOBAL_ZIG_VER
ARG GLOBAL_TARGET
ARG GLOBAL_USER

WORKDIR /ziglang

RUN --mount=type=cache,target=/ziglang \
    wget https://ziglang.org/download/0.11.0/$GLOBAL_ZIG_VER.tar.xz \
    && tar -xvf $GLOBAL_ZIG_VER.tar.xz

WORKDIR /usr/bin

RUN --mount=type=cache,target=\ziglang \
    ln -s /ziglang/${GLOBAL_ZIG_VER}/zig zig

RUN cargo install cargo-zigbuild

WORKDIR /app

RUN --mount=type=cache,target=/ziglang \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo zigbuild --target $GLOBAL_TARGET --release --package backend

FROM gcr.io/distroless/cc AS RUNNER

ARG GLOBAL_TARGET
ARG DB_HOST=localhost
ARG DB_USERNAME=postgres
ARG DB_PASSWORD=postgres

ENV DB_HOST=$DB_HOST
ENV DB_USERNAME=$DB_USERNAME
ENV DB_PASSWORD=$DB_PASSWORD

WORKDIR /app

COPY --from=builder /app/target/$GLOBAL_TARGET/release/backend ./

EXPOSE 8080

CMD ["/app/backend"]

