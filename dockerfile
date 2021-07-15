FROM rust:1.53

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release

COPY . /
ENV USER_TABLE_NAME=api-server-rust-stack-user-table
RUN cargo build --release
CMD ["/target/release/api-server-rs"]