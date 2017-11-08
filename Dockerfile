FROM rust:1.23

ADD . /app
WORKDIR /app

RUN rustup default nightly \
    && cargo install

EXPOSE 8000

CMD ["jongleur_back"]
