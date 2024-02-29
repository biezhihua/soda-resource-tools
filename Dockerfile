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


# 安装SSH服务
RUN apt-get update && apt-get install -y openssh-server
RUN mkdir /var/run/sshd

# 设置SSH访问时的欢迎信息（可选）
RUN echo 'Welcome to your Docker container!' > /etc/motd

# 添加一个用户（替换your_user和your_password为你想设置的用户名和密码）
RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 ubuntu
RUN  echo 'ubuntu:password' | chpasswd

# 允许root用户登录（可选，根据需要决定是否启用）
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# SSH登录时不询问密钥确认（可选，但对某些自动化操作有帮助）
RUN sed -ri 's/#HostKey \/etc\/ssh\/ssh_host_rsa_key/HostKey \/etc\/ssh\/ssh_host_rsa_key/g' /etc/ssh/sshd_config
RUN sed -ri 's/#HostKey \/etc\/ssh\/ssh_host_ecdsa_key/HostKey \/etc\/ssh\/ssh_host_ecdsa_key/g' /etc/ssh/sshd_config
RUN sed -ri 's/#HostKey \/etc\/ssh\/ssh_host_ed25519_key/HostKey \/etc\/ssh\/ssh_host_ed25519_key/g' /etc/ssh/sshd_config
RUN sed -ri 's/#PasswordAuthentication yes/PasswordAuthentication yes/g' /etc/ssh/sshd_config

# 开放22端口
EXPOSE 22

COPY --from=builder /usr/src/soda/target/release/soda_clix /usr/local/bin/soda_clix
RUN chmod +rx /usr/local/bin/soda_clix && ls -l /usr/local/bin/soda_clix
# CMD [ "/usr/local/bin/soda_clix --version" ]

# 启动SSH服务
CMD ["/usr/sbin/sshd", "-D"] && tail -f /dev/null