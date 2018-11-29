FROM buildpack-deps:stretch
MAINTAINER mairandomness <mairakodama@gmail.com>

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt install libssl-dev

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN cargo install diesel_cli --no-default-features --features "postgres"
EXPOSE 8080

ENV SOURCES=/sources

RUN mkdir -p $SOURCES
ADD ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release
RUN diesel migration run

CMD ROCKET_ENV=production ./target/release/main