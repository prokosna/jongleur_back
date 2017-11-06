FROM rust:1.21

ADD . /app
WORKDIR /app

RUN apt update \
    && apt install -y libmongoc-1.0-0 \
    && rustup default nightly \
    && cargo install

EXPOSE 8000

CMD ["jongleur_back"]
