#!/bin/bash
RUSTFLAGS="-C target-cpu=x86-64-v2"
cargo build --release