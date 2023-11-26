FROM rust:slim-buster
WORKDIR /usr/src/flora-api
COPY . .
EXPOSE 8080
RUN apt update && apt install pkg-config libssl-dev
RUN cargo install --path .
CMD ["flora-api"]