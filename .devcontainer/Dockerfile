FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-22.04

RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy main restricted universe multiverse" > /etc/apt/sources.list
RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-security main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-updates main restricted universe multiverse" >> /etc/apt/sources.list
RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ jammy-backports main restricted universe multiverse" >> /etc/apt/sources.list

RUN apt-get update -y
RUN apt-get install -y language-pack-zh-hans
ENV TZ Asia/Shanghai
ENV LANG zh_CN.UTF-8

RUN apt-get install -y pkg-config build-essential autoconf make
RUN apt-get install -y gcc
RUN apt-get install -y libssh-dev

RUN curl https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init > rustup-init && \
        chmod 744 rustup-init && \
        ./rustup-init -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly
RUN cargo install cargo-expand cargo-cache