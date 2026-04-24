FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
ENV RUSTFLAGS="-C target-cpu=haswell"
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:trixie-slim
WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fraud-detection /usr/local/bin/api

COPY --from=builder /app/resources/references.bin ./resources/
COPY --from=builder /app/resources/mcc_risk.json ./resources/
COPY --from=builder /app/resources/normalization.json ./resources/

EXPOSE 3000

CMD ["api"]
