FROM rust:1.81.0-alpine3.19

RUN apk add musl-dev

WORKDIR /usr/src/rbg-ivy
COPY . .

RUN cargo install --path .

EXPOSE 8080
CMD ["rgb-ivy"]
