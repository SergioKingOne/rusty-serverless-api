FROM amazonlinux:2

RUN yum update -y && \
    yum install -y gcc zip openssl-devel perl

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app 