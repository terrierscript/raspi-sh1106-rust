FROM rust:1.35

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf

CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf", "--release"]