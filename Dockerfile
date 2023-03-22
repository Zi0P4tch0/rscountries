FROM rust:1.68 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/rscountries /usr/local/bin/rscountries
CMD ["rscountries"]
