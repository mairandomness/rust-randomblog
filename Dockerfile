FROM debian:jessie AS builder

RUN apt-get update && apt-get install -y curl libpq-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain none

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup toolchain install nightly --allow-downgrade --profile minimal

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
CMD ROCKET_PORT=$PORT /usr/local/bin/main