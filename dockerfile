FROM rust:1.53

COPY ./target/release/api-server-rs .
CMD ["./api-server-rs"]