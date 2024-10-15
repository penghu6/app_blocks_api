# 构建阶段
FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# 运行阶段
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/app_blocks_api /usr/local/bin/
EXPOSE 10940
CMD ["app_blocks_api"]