FROM rust:1.81-alpine AS builder
WORKDIR /src
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# 自动检测目标架构
RUN TARGET_ARCH=$(uname -m) && \
    if [ "$TARGET_ARCH" = "aarch64" ]; then \
        export RUST_TARGET="aarch64-unknown-linux-musl"; \
    else \
        export RUST_TARGET="x86_64-unknown-linux-musl"; \
    fi && \
    echo "Building for $RUST_TARGET" && \
    rustup target add $RUST_TARGET && \
    cargo build --release --locked --target $RUST_TARGET && \
    # 复制二进制文件到固定位置
    cp target/$RUST_TARGET/release/git-green /tmp/git-green

FROM alpine:3.20
RUN apk add --no-cache git ca-certificates
WORKDIR /app
COPY --from=builder /tmp/git-green /usr/local/bin/git-green
CMD ["git-green"]