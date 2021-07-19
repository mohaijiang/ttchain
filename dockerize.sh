#!/bin/bash
REGISTRY="183.66.65.207:5000"
PACKAGE_VERSION=1.0.0
IMAGEID="$REGISTRY/ttchain/miner-worker:$PACKAGE_VERSION"
echo "Building $REGISTRY/ttchain/miner-worker:$PACKAGE_VERSION ..."
docker build -t $IMAGEID .
#
#while [ -n "$1" ]
#    do
#        case "$1" in
#            -p|--push)
#                docker push $IMAGEID
#                break
#                ;;
#            --)
#                shift
#                break
#                ;;
#            *)
#                return 0
#                ;;
#        esac
#    done