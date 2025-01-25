FROM rust:alpine AS builder
RUN apk add musl-dev
WORKDIR /worlspace
COPY Cargo.lock Cargo.toml dummy.rs ./
RUN mkdir .cargo && cargo vendor > .cargo/config.toml && cargo build --bin dummy --release
COPY src/ src/
RUN cargo build --release

FROM scratch

ENV ROCKET_TEMPLATE_DIR="/templates"
COPY --from=build-env /workspace/templates /templates
COPY --from=builder /workspace/target/release/mail-autodiscover-autoconfig /mail-autodiscover-autoconfig

EXPOSE 8000

ENTRYPOINT [ "/mail-autodiscover-autoconfig" ]
