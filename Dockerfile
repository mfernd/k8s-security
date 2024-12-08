FROM clux/muslrust:stable AS base

RUN apt update && \
    apt upgrade

FROM base AS build

WORKDIR /app

COPY ./Cargo.* .
COPY ./crates/ ./crates/
RUN cargo fetch

COPY ./.data ./.data
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch AS aggregator-svc

USER 1000

COPY --from=build --chown=1000 /app/target/x86_64-unknown-linux-musl/release/aggregator-svc /

ENV APP_PORT=80
EXPOSE ${APP_PORT}

ENV RUST_LOG="tower_http=trace,info"
CMD ["/aggregator-svc"]

FROM scratch AS provider-svc

USER 1000

COPY --from=build --chown=1000 /app/target/x86_64-unknown-linux-musl/release/provider-svc /

ENV APP_PORT=80
EXPOSE ${APP_PORT}

ENV RUST_LOG="tower_http=trace,info"
CMD ["/provider-svc"]
