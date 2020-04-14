#!/bin/bash

function cleanup {
    kill "$ACCOUNTS_PID"
    kill "$PRODUCTS_PID"
    kill "$REVIEWS_PID"
}
trap cleanup EXIT

cargo build --bin federation-accounts
cargo build --bin federation-products
cargo build --bin federation-reviews

cargo run --bin federation-accounts &
ACCOUNTS_PID=$!

cargo run --bin federation-products &
PRODUCTS_PID=$!

cargo run --bin federation-reviews &
REVIEWS_PID=$!

sleep 3

node index.js
