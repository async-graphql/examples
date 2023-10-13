#!/bin/bash

set -eumo pipefail

function cleanup {
  for pid in "${PRODUCTS_ROVER_PID:-}" "${REVIEWS_ROVER_PID:-}" "${ACCOUNTS_PID:-}" "${PRODUCTS_PID:-}" "${REVIEWS_PID:-}"; do
    # try kill all registered pids
    [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null && kill "$pid" || echo "Could not kill $pid"
  done
}
trap cleanup EXIT

cargo build --bin static-federation-accounts
cargo build --bin static-federation-products
cargo build --bin static-federation-reviews

cargo run --bin static-federation-accounts &
ACCOUNTS_PID=$!

cargo run --bin static-federation-products &
PRODUCTS_PID=$!

cargo run --bin static-federation-reviews &
REVIEWS_PID=$!

sleep 3

rover dev --url http://localhost:4001 --name accounts &
sleep 1
rover dev --url http://localhost:4002 --name products &
PRODUCTS_ROVER_PID=$!
sleep 1
rover dev --url http://localhost:4003 --name reviews &
REVIEWS_ROVER_PID=$!
fg %4