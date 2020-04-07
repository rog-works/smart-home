FROM rust:1-slim-buster

RUN useradd -m worker

USER worker
ENV USER=worker

WORKDIR /opt/app

CMD tail -f /dev/null
