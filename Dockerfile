FROM ubuntu:18.04

RUN apt-get update
RUN apt-get -y install curl gcc build-essential pkg-config openssl libssl-dev libpq-dev
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

RUN apt-get update
RUN apt-get -y install openssl libssl-dev libpq-dev
COPY --from=0 /rust/target/release/rocket-timer /app/bin/rocket-timer
COPY --from=0 /rust/Rocket.toml /app/bin/Rocket.toml

EXPOSE 8000
CMD /app/bin/rocket-timer
