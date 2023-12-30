#!/usr/bin/env bash

error() {
  echo "Error: $1" >&2 && exit 1
}

if [[ "$#" -ne 1 ]]; then
  error "There must be exactly one argument"
fi

if ! [[ $1 =~ ^[1-9][0-9]*$ ]]; then
  error "Not a valid number"
fi

cargo run --bin "problem_$1" --release
