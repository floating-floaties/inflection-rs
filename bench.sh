#!/bin/sh

set -e
cargo test
cargo bench -- --plotting-backend gnuplot
