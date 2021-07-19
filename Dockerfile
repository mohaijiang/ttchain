# 分阶段构建
FROM paritytech/ci-linux:363245ca-20210706 as build

# rust 镜像源配置
ADD ./.cargo/config /root/.cargo/config

# Create ttchain directory
WORKDIR /tmp

# Move source files to docker image
COPY . .

# Run Build
RUN cargo build --release

## 镜像构建
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y openssl

WORKDIR /opt/ttchain/

COPY ./docker/run.sh /opt/run.sh
COPY --from=build /tmp/target/release/node-template /opt/ttchain/node-template

CMD /opt/run.sh
