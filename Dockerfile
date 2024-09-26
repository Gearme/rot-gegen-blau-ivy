FROM rust:1.67

WORKDIR /usr/src/rbg-ivy
COPY . .

RUN cargo install --path .

EXPOSE 8080
CMD ["rgb-ivy"]
