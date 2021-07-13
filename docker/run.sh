
#! /usr/bin/env bash
echo "Starting ttchain using args: ${ARGS}"

cd /opt/ttchain
/opt/ttchain/node-template ${ARGS}
