run: chainspec
	bunx @acala-network/chopsticks@latest --config poc/runtime/chopsticks.yml --genesis output/chainspec.json

poc-host: poc-guest
	RUST_LOG=trace cargo run -p poc-host

poc-guest:
	cd poc/guest; RUSTFLAGS="-C relocation-model=pie -C link-arg=--emit-relocs -C link-arg=--unique --remap-path-prefix=$(pwd)= --remap-path-prefix=$$HOME=~" cargo build -q --release --bin poc-guest -p poc-guest
	mkdir -p output
	polkatool link --run-only-if-newer -s poc/guest/target/riscv32ema-unknown-none-elf/release/poc-guest -o output/poc-guest.polkavm

tools: polkatool chain-spec-builder

polkatool:
	cargo install --git https://github.com/koute/polkavm polkatool

chain-spec-builder:
	cargo install --git https://github.com/paritytech/polkadot-sdk --tag polkadot-v1.9.0 staging-chain-spec-builder

fmt:
	cargo fmt --all

check:
	SKIP_WASM_BUILD= cargo check --no-default-features --target=wasm32-unknown-unknown
	SKIP_WASM_BUILD= cargo check
	cd poc/guest; cargo check

clippy:
	SKIP_WASM_BUILD= cargo clippy -- -D warnings
	cd poc/guest; cargo clippy

chainspec:
	cargo build -p poc-runtime --release
	mkdir -p output
	cp target/release/wbuild/poc-runtime/poc_runtime.compact.compressed.wasm output
	chain-spec-builder -c output/chainspec.json create -n poc-runtime -i poc-runtime -r ./output/poc_runtime.compact.compressed.wasm -s default
	cat output/chainspec.json | jq '.properties = {}' > output/chainspec.json.tmp
	mv output/chainspec.json.tmp output/chainspec.json
