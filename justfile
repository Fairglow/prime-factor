# Just recipes for AgeCalc
alias b := build
alias bud := build
alias ben := bench
alias r := release
alias rel := release
alias t := tests
alias tes := tests

all: build test release

bench:
    cargo bench

build:
    cargo build --all-targets && cargo clippy

release:
    cargo build --release

test:
    cargo nextest run --test-threads num-cpus

test-out:
    #cargo test -- --nocapture
    cargo nextest run --no-capture --test-threads num-cpus

test-rel:
    cargo nextest run --release --test-threads num-cpus

tests: test test-rel
