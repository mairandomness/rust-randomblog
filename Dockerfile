FROM debian:jessie AS builder

RUN apt-get update && apt-get install -y curl libpq-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly-2020-08-12

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:jessie

RUN apt-get update && apt-get install -y libpq-dev

COPY --from=builder \
  /target/release/main \
  /usr/local/bin/
COPY --from=builder \
  /templates /root/templates
COPY --from=builder \
  /static /root/static
COPY --from=builder \
  /Rocket.toml /root/

WORKDIR /root
CMD ROCKET_PORT=8080 /usr/local/bin/main