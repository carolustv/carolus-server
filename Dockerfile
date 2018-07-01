FROM rust:latest

ADD . /

RUN rustup default nightly

RUN cargo build --release

EXPOSE 8000

ENTRYPOINT ["/target/release/carolus"]
