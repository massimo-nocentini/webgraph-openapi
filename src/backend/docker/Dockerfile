
FROM rust:alpine

RUN apk add --no-cache musl-dev

COPY ./src /usr/src/backend/src
COPY ./Cargo* /usr/src/backend

COPY ./data/erdos-renyi /usr/webgraphs/egonet-erdos-renyi

WORKDIR /usr/src/backend

RUN cargo build --release

EXPOSE 8000

ENTRYPOINT [ "cargo", "run", "--release" ]