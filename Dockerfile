FROM rust:1-buster
WORKDIR /usr/src/user-service

COPY . .

RUN cargo build --release

RUN cargo install --path .
CMD ["/usr/local/cargo/bin/user-service"]
