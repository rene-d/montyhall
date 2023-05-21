#!/bin/sh

# https://doc.rust-lang.org/rustc/instrument-coverage.html

set -e


cargo install rustfilt

cargo clean
RUSTFLAGS="-C instrument-coverage" cargo build --bin montyhall6

rm -f *.profraw *.profdata

LLVM_PROFILE_FILE="montyhall6.profraw" target/debug/montyhall6 -t 1000000

PATH=/Library/Developer/CommandLineTools/usr/bin/:$PATH

llvm-profdata merge -sparse montyhall6.profraw -o montyhall6.profdata
llvm-cov show --format=html -o coverage -Xdemangler=rustfilt target/debug/montyhall6 \
    -instr-profile=montyhall6.profdata \
    -show-line-counts-or-regions \
    -show-instantiations \
    --ignore-filename-regex=/.cargo/registry

open coverage/index.html
