# 阶段1: 构建阶段
FROM rust:latest as builder
RUN apt-get update && \
    apt-get install -y openssl libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/soda
COPY . .
RUN cargo build --release

# 阶段2: 最终镜像
FROM ubuntu:latest
ARG DEBIAN_FRONTEND=noninteractive
COPY --from=builder /usr/src/soda/target/release/soda_clix /usr/local/bin/soda_clix
RUN chmod +rx /usr/local/bin/soda_clix && ls -l /usr/local/bin/soda_clix
# CMD [ "/usr/local/bin/soda_clix --version" ]
CMD tail -f /dev/null