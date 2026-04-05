FROM rust:1.76-bullseye
WORKDIR /usr/src/gotham-gateway
COPY . .
RUN cargo build --release
CMD ["./target/release/gotham-gateway"]
