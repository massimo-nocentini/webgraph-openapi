
FROM rust:1.87-alpine

RUN apk add --no-cache musl-dev

COPY ./src /usr/src/backend/src

COPY ./data/erdos-renyi /usr/webgraphs/egonet-erdos-renyi

COPY ./Cargo* /usr/src/backend

WORKDIR /usr/src/backend

RUN cargo build --release

EXPOSE 8000

ENTRYPOINT [ "cargo", "run", "--release" ]