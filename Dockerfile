FROM heroku/cedar:14

RUN useradd -d /app -m app

WORKDIR /app

ENV SHELL /bin/sh

RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --disable-sudo --channel=nightly -y

USER app

RUN mkdir -p /app/src
RUN mkdir -p /app/.profile.d

ONBUILD COPY . /app/src/

USER root

WORKDIR /app/src

ONBUILD RUN cargo fetch --verbose
ONBUILD RUN cargo build --release

ONBUILD EXPOSE 8080
