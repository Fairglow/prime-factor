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
    unbuffer cargo bench | tee bench.log

build: lint
    unbuffer cargo build --all-targets | tee build.log

lint:
    unbuffer cargo clippy | tee lint.log

outdated:
    cargo outdated --depth=1

reikna:
    unbuffer cargo bench --bench bench-reikna --features bench-reikna | tee compare.log

release:
    cargo build --release

test:
    unbuffer cargo nextest run --test-threads num-cpus | tee test.log

test-out:
    cargo nextest run --no-capture --test-threads num-cpus

test-rel:
    cargo nextest run --release --test-threads num-cpus

tests: test-rel
