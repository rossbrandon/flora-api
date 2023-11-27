FROM rust:slim-buster
WORKDIR /usr/src/flora-api
COPY . .
EXPOSE 8080
RUN apt-get update && apt-get install -y libssl-dev
RUN cargo install --path .
CMD ["flora-api"]