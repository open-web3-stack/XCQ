poc-host: poc-guest
	cargo run -p poc-host

poc-guest:
	cd poc/guest; RUSTFLAGS="-C relocation-model=pie -C link-arg=--emit-relocs -C link-arg=--unique --remap-path-prefix=$(pwd)= --remap-path-prefix=$HOME=~" cargo build -q --release --bin poc-guest -p poc-guest
	mkdir -p output
	polkatool link --run-only-if-newer -s target/riscv32ema-unknown-none-elf/release/poc-guest -o output/poc-guest.polkavm

polkatool:
	cargo install --git https://github.com/koute/polkavm --force polkatool