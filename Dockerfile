ARG DATABASE_URL
ARG MI_REDIS_URL
ARG MI_AUTH_REDIRECT_URI
ARG OSU_CLIENT_ID
ARG OSU_CLIENT_SECRET
ARG OSU_REDIRECT_URI
ARG PORT
ARG RUST_LOG

FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /usr/src/mapper-influence

FROM node:16 as ui-builder
WORKDIR /usr/src/mapper-influence
COPY . .

RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
RUN just export-ui 

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /usr/src/mapper-influence/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.17

RUN addgroup -S myuser && adduser -S myuser -G myuser

WORKDIR /app

COPY --from=builder /usr/src/mapper-influence/target/x86_64-unknown-linux-musl/release/mi-api /usr/local/bin
COPY --from=ui-builder /usr/src/mapper-influence/mi-api/pages /app/pages

USER myuser

ENV DATABASE_URL=${DATABASE_URL}
ENV MI_REDIS_URL=${MI_REDIS_URL}
ENV MI_AUTH_REDIRECT_URI=${MI_AUTH_REDIRECT_URI}
ENV OSU_CLIENT_ID=${OSU_CLIENT_ID}
ENV OSU_CLIENT_SECRET=${OSU_CLIENT_SECRET}
ENV OSU_REDIRECT_URI=${OSU_REDIRECT_URI}
ENV PORT=${PORT}
ENV RUST_LOG=${RUST_LOG}

EXPOSE ${PORT}

CMD [ "mi-api" ]
