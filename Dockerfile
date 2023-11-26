FROM rust:slim-buster
WORKDIR /usr/src/flora-api
COPY . .
EXPOSE 8080
RUN sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev
RUN cargo install --path .
CMD ["flora-api"]