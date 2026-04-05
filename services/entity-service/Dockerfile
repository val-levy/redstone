FROM rust:1.76-bullseye
WORKDIR /usr/src/entity-service
COPY . .
RUN cargo build --release
CMD ["./target/release/entity-service"]
