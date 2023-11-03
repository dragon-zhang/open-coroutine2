#!/usr/bin/env sh

set -ex

CARGO=cargo
if [ "${CROSS}" = "1" ]; then
    export CARGO_NET_RETRY=5
    export CARGO_NET_TIMEOUT=10

    cargo install cross
    CARGO=cross
fi

# If a test crashes, we want to know which one it was.
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1

# test open-coroutine-core mod
cd "${PROJECT_DIR}"/open-coroutine-core
"${CARGO}" test --target "${TARGET}" --no-default-features --features korosensei
"${CARGO}" test --target "${TARGET}" --no-default-features --features korosensei --release

"${CARGO}" test --target "${TARGET}" --no-default-features --features net
"${CARGO}" test --target "${TARGET}" --no-default-features --features net --release

if [ "${TARGET}" = "x86_64-unknown-linux-gnu" ]; then
    "${CARGO}" test --target "${TARGET}" --no-default-features --features io_uring
    "${CARGO}" test --target "${TARGET}" --no-default-features --features io_uring --release
fi

if [ "${TARGET}" != "riscv64gc-unknown-linux-gnu" ]; then
    "${CARGO}" test --target "${TARGET}" --no-default-features --features preemptive-schedule
fi

"${CARGO}" test --target "${TARGET}" --no-default-features --features preemptive-schedule --release

# test examples
cd "${PROJECT_DIR}"/examples
"${CARGO}" run --example sleep_not_co --release
"${CARGO}" run --example sleep_co --release
#"${CARGO}" run --example socket_not_co --release
"${CARGO}" run --example socket_co_server --release
"${CARGO}" run --example socket_co_client --release
"${CARGO}" run --example socket_co --release
