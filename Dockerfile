FROM rust:slim-buster
WORKDIR /usr/src/flora-api
COPY . .
EXPOSE 8080
RUN cargo install --path .
CMD ["flora-api"]