FROM rust:1.76-bullseye
WORKDIR /usr/src/graph-service
COPY . .
RUN cargo build --release
CMD ["./target/release/graph-service"]
