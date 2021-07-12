# Build crust image
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y openssl
COPY ./target/release/node-template /opt/ttchain/node-template
COPY run.sh /opt/run.sh

WORKDIR /opt/ttchain/
CMD /opt/run.sh
