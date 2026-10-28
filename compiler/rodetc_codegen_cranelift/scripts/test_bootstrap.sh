#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/../"

source ./scripts/setup_rust_fork.sh

echo "[TEST] Bootstrap of rustc"
pushd rust
rm -r compiler/rodetc_codegen_cranelift/{Cargo.*,src}
cp ../Cargo.* compiler/rodetc_codegen_cranelift/
cp -r ../src compiler/rodetc_codegen_cranelift/src

./x.py build --stage 1 library/std
popd
