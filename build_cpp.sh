#!/bin/bash

BASE_DIR=$(pwd)
pushd $BASE_DIR >/dev/null

while [[ "$BASE_DIR" != "/" ]]; do
    BASE_DIR=$(pwd)
    [ -d .git ] && break
    cd ..
done

mkdir -p "$BASE_DIR/build" && pushd "$BASE_DIR/build" >/dev/null

g++ "$BASE_DIR/cpp_src"/*.cpp -lstdc++ -std=c++14 -o cpp_example

echo "Built executable $(pwd)/cpp_example"

popd >/dev/null
popd >/dev/null
