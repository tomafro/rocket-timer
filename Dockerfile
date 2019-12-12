FROM ubuntu:18.04

RUN apt-get update
RUN apt-get -y install curl gcc build-essential pkg-config libssl-dev libpq-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain nightly

ENV USER=root
ENV PATH=/root/.cargo/bin:$PATH

RUN cd / && cargo new rust
WORKDIR /rust
ADD Cargo.toml /rust/Cargo.toml
ADD Cargo.lock /rust/Cargo.lock
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM ubuntu:18.04

RUN apt-get update && apt-get -y install libpq-dev && apt-get clean
COPY --from=0 /rust/target/release/rocket-timer /app/bin/rocket-timer
COPY --from=0 /rust/Rocket.toml /app/bin/Rocket.toml

EXPOSE 8000
ENTRYPOINT ["/app/bin/rocket-timer"]
