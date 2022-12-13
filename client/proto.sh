#!/bin/bash

if [[ ! -d ./proto ]]
then
    mkdir ./proto
fi
python -m grpc_tools.protoc \
    -I../proto \
    --python_out=. \
    --pyi_out=. \
    --grpc_python_out=. \
../proto/hello.proto