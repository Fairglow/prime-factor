# Just recipes for prime-factor
alias b := build
alias bud := build
alias ben := bench
alias old := outdated
alias r := release
alias rel := release
alias t := tests

all: build test release

bench:
    cargo bench

build:
    cargo build --all-targets && cargo clippy

outdated:
    cargo outdated --depth=1

release:
    cargo build --release

test:
    cargo nextest run --test-threads num-cpus

test-out:
    cargo nextest run --no-capture --test-threads num-cpus

test-rel:
    cargo nextest run --release --test-threads num-cpus

tests: test-rel
