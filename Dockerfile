# docker to build rust for linux ubuntu

FROM ubuntu:18.04

# install rust
RUN apt-get update && apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get install -y  gcc 

# musl-gcc
RUN apt-get install -y musl-tools
RUN apt-get install -y musl-dev 

# install openssl
RUN apt-get install -y pkg-config libssl-dev

# openssl
# RUN apt-get install -y openssl
RUN apt install pkg-config
RUN apt-get install libudev-dev
# set OPENSSL_DIR env
# ENV OPENSSL_DIR=/usr/include/openssl

# volumne is .
# VOLUME [ "/app" ]
WORKDIR /app

# run build
CMD cargo build --release --target=x86_64-unknown-linux-musl && mv target/x86_64-unknown-linux-musl/release/app /app/app


# echo cwd
# CMD pwd && ls -la && cargo build --release --target=x86_64-unknown-linux-musl
# docker build -t rust-build .

# remove the container
# docker rm -f rust-build

# wait foreevr
# docker run -it --name rust-build rust-build bash
# CMD tail -f /dev/null


