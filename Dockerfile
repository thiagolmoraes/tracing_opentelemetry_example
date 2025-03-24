FROM rust:1.85-slim-bullseye AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim AS runner
WORKDIR /app

RUN apt-get update && apt-get install -y \
     libssl1.1 \
     ca-certificates \
     libssl-dev \
     openssl \
     curl     \
     && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tracing_opentelemetry_example  .

ENTRYPOINT ["./tracing_opentelemetry_example"]
