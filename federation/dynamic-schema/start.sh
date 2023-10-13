#!/bin/bash

set -eumo pipefail

function cleanup {
  kill "$PRODUCTS_ROVER_PID"
  kill "$REVIEWS_ROVER_PID"
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

cargo build --bin dynamic-federation-accounts
cargo build --bin dynamic-federation-products
cargo build --bin dynamic-federation-reviews

cargo run --bin dynamic-federation-accounts &
ACCOUNTS_PID=$!

cargo run --bin dynamic-federation-products &
PRODUCTS_PID=$!

cargo run --bin dynamic-federation-reviews &
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
