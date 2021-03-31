FROM rust:1.48.0

WORKDIR /usr/src/restapp
COPY . .

RUN cargo build

EXPOSE 11111

CMD ["cargo", "run"]
