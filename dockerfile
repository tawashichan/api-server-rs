FROM rust:1.53 as build

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release

COPY . /
RUN cargo build --release

FROM gcr.io/distroless/cc

ENV USER_TABLE_NAME=api-server-rust-stack-user-table
COPY --from=build /target/release/api-server-rs .
CMD ["./api-server-rs"]