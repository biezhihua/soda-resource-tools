# 使用官方Rust镜像作为基础镜像
FROM rust:latest

# 安装OpenSSL
RUN apt-get update && \
    apt-get install -y openssl libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# install soda_clix
RUN cargo install soda_clix

# test soda_clix
RUN soda_clix --version