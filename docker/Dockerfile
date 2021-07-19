
# Build crust image
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y openssl

WORKDIR /opt/ttchain/

COPY run.sh /opt/run.sh
COPY node-template /opt/ttchain/node-template

COPY customSpecRaw.json /opt/ttchain/customSpecRaw.json

CMD /opt/run.sh
