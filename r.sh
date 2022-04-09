#!/bin/zsh
if [ -x "$(command -v cargo-nextest)" ]; then
  cargo nextest run "$@"; 
else
  cargo test "$@"; 
fi
