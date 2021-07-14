FROM rust:1.53

COPY . ./
RUN cargo build --release
CMD ["./target/release/api-server-rs"]