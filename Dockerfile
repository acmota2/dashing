FROM rust:1.89 AS builder

WORKDIR /usr/src/dashing
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/src/dashing/target/release/dashing /usr/local/bin/dashing
COPY config.json /etc/dashing/config.json
COPY assets /assets

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/dashing"]
