FROM rust:1.95 AS builder

WORKDIR /usr/src/dashing
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian13 as runner

COPY --from=builder --chown=nonroot:nonroot /usr/src/dashing/target/release/dashing /usr/local/bin/dashing
COPY --chown=nonroot:nonroot assets /assets

EXPOSE 8080

USER nonroot:nonroot

ENTRYPOINT ["/usr/local/bin/dashing"]
