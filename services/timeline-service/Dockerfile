FROM rust:1.76-bullseye
WORKDIR /usr/src/timeline-service
COPY . .
RUN cargo build --release
CMD ["./target/release/timeline-service"]
