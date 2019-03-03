#!/bin/bash
cargo watch -w src -w Cargo.toml -x build -x test -x doc
