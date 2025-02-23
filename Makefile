run: chainspec
	bunx @acala-network/chopsticks@latest --config poc/runtime/chopsticks.yml --genesis output/chainspec.json

poc-guests: poc-guest-sum-balance poc-guest-sum-balance-percent poc-guest-total-supply poc-guest-transparent-call

dummy-poc-guests: dummy-poc-guest-sum-balance dummy-poc-guest-sum-balance-percent dummy-poc-guest-total-supply dummy-poc-guest-transparent-call

poc-guest-%:
	cd poc/guests; RUSTFLAGS="-D warnings" cargo build -q --release -Z build-std=core,alloc --target "../../vendor/polkavm/crates/polkavm-linker/riscv32emac-unknown-none-polkavm.json" --bin poc-guest-$* -p poc-guest-$*
	mkdir -p output
	polkatool link --run-only-if-newer -s poc/guests/target/riscv32emac-unknown-none-polkavm/release/poc-guest-$* -o output/poc-guest-$*.polkavm

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
	cargo check --no-default-features --target=wasm32-unknown-unknown -p pvq-api -p pvq-executor -p pvq-extension-core -p pvq-extension-fungibles -p pvq-extension -p pvq-primitives -p pvq-runtime-api
	SKIP_WASM_BUILD= cargo check --no-default-features --target=wasm32-unknown-unknown -p poc-runtime

check: check-wasm
	SKIP_WASM_BUILD= cargo check
	cd poc/guests; cargo check

clippy:
	SKIP_WASM_BUILD= cargo clippy -- -D warnings
	cd poc/guests; RUSTFLAGS="-D warnings" cargo clippy -Z build-std=core,alloc --target "../../vendor/polkavm/crates/polkavm-linker/riscv32emac-unknown-none-polkavm.json" --all

test:
	SKIP_WASM_BUILD= cargo test

chainspec:
	cargo build -p poc-runtime --release
	mkdir -p output
	cp target/release/wbuild/poc-runtime/poc_runtime.compact.compressed.wasm output
	chain-spec-builder -c output/chainspec.json create -n poc-runtime -i poc-runtime -r ./output/poc_runtime.compact.compressed.wasm -s default
	cat output/chainspec.json | jq '.properties = {}' > output/chainspec.json.tmp
	mv output/chainspec.json.tmp output/chainspec.json
