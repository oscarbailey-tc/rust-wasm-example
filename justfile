TARGET:="wasm32-unknown-unknown"
BINARY:="target/" + TARGET + "/release/rust_wasm.wasm"

build:
    cargo build --target {{TARGET}} --release
    wasm-strip {{BINARY}}
    mkdir -p www
    wasm-opt -o www/lib.wasm -Oz {{BINARY}}
    ls -lh www/lib.wasm

leaks:
    cd wasm-leaks && cargo build
    valgrind wasm-leaks/target/debug/wasm-leaks
