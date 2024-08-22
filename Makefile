GUEST_RUST_FLAGS="-C relocation-model=pie -C link-arg=--emit-relocs -C link-arg=--unique --remap-path-prefix=$(pwd)= --remap-path-prefix=$HOME=~"

run: chainspec
	bunx @acala-network/chopsticks@latest --config poc/runtime/chopsticks.yml --genesis output/chainspec.json

poc-guests: poc-guest-sum-balance poc-guest-sum-balance-percent poc-guest-total-supply poc-guest-transparent-call

dummy-poc-guests: dummy-poc-guest-sum-balance dummy-poc-guest-sum-balance-percent dummy-poc-guest-total-supply dummy-poc-guest-transparent-call

poc-guest-%:
	cd poc/guests; RUSTFLAGS=$(GUEST_RUST_FLAGS) cargo build -q --release --bin poc-guest-$* -p poc-guest-$*
	mkdir -p output
	polkatool link --run-only-if-newer -s poc/guests/target/riscv32ema-unknown-none-elf/release/poc-guest-$* -o output/poc-guest-$*.polkavm

dummy-poc-guest-%:
	mkdir -p output
	touch output/poc-guest-$*.polkavm

tools: polkatool chain-spec-builder

polkatool:
	cargo install --path vendor/polkavm/tools/polkatool

chain-spec-builder:
	cargo install --git https://github.com/paritytech/polkadot-sdk --tag polkadot-v1.12.0 staging-chain-spec-builder

fmt:
	cargo fmt --all

check-wasm:
	cargo check --no-default-features --target=wasm32-unknown-unknown -p xcq-api -p xcq-executor -p xcq-extension-core -p xcq-extension-fungibles -p xcq-extension -p xcq-primitives -p xcq-runtime-api -p xcq-types
	SKIP_WASM_BUILD= cargo check --no-default-features --target=wasm32-unknown-unknown -p poc-runtime

check: check-wasm
	SKIP_WASM_BUILD= cargo check
	cd poc/guests; cargo check

clippy:
	SKIP_WASM_BUILD= cargo clippy -- -D warnings
	cd poc/guests; cargo clippy

test:
	SKIP_WASM_BUILD= cargo test

chainspec:
	cargo build -p poc-runtime --release
	mkdir -p output
	cp target/release/wbuild/poc-runtime/poc_runtime.compact.compressed.wasm output
	chain-spec-builder -c output/chainspec.json create -n poc-runtime -i poc-runtime -r ./output/poc_runtime.compact.compressed.wasm -s default
	cat output/chainspec.json | jq '.properties = {}' > output/chainspec.json.tmp
	mv output/chainspec.json.tmp output/chainspec.json
