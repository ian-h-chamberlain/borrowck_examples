#!/bin/bash

BASE_DIR=$(pwd)
pushd $BASE_DIR

while [[ "$BASE_DIR" != "/" ]]; do
    BASE_DIR=$(pwd)
    [ -d .git ] && break
    cd ..
done

pushd "$BASE_DIR/build"

g++ "$BASE_DIR/cpp_src"/*.cpp -lstdc++ -std=c++14 -o cpp_example

popd
popd
