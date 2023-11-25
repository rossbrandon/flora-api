FROM rust:slim-buster
WORKDIR /usr/src/flora-api
COPY . .
RUN cargo install --path .
CMD ["flora-api"]