run: chainspec
	bunx @acala-network/chopsticks@latest --config poc/runtime/chopsticks.yml --genesis output/chainspec.json

GUEST_EXAMPLES = $(shell find guest-examples -name "Cargo.toml" -not -path "guest-examples/Cargo.toml" | xargs -n1 dirname | xargs -n1 basename)
GUEST_TARGETS = $(patsubst %,guest-%,$(GUEST_EXAMPLES))
DUMMY_GUEST_TARGETS = $(patsubst %,dummy-guest-%,$(GUEST_EXAMPLES))

guests: $(GUEST_TARGETS)

dummy-guests: $(DUMMY_GUEST_TARGETS)

guest-%:
	cd guest-examples; cargo build -q --release --bin guest-$* -p guest-$*
	mkdir -p output
	polkatool link --run-only-if-newer -s guest-examples/target/riscv32emac-unknown-none-polkavm/release/guest-$* -o output/guest-$*.polkavm

dummy-guest-%:
	mkdir -p output
	touch output/guest-$*.polkavm

tools: polkatool chain-spec-builder

polkatool:
	cargo install --path vendor/polkavm/tools/polkatool

chain-spec-builder:
	cargo install --git https://github.com/paritytech/polkadot-sdk --tag polkadot-v1.12.0 staging-chain-spec-builder

fmt:
	cargo fmt --all

check-wasm:
	cargo check --no-default-features --target=wasm32-unknown-unknown -p pvq-program -p pvq-executor -p pvq-extension-core -p pvq-extension-fungibles -p pvq-extension -p pvq-primitives -p pvq-runtime-api
	SKIP_WASM_BUILD= cargo check --no-default-features --target=wasm32-unknown-unknown -p poc-runtime

check: check-wasm
	SKIP_WASM_BUILD= cargo check
	cd pvq-program/examples; cargo check

clippy:
	SKIP_WASM_BUILD= cargo clippy -- -D warnings
	cd guest-examples; cargo clippy --all

test:
	SKIP_WASM_BUILD= cargo test

chainspec:
	cargo build -p poc-runtime --release
	mkdir -p output
	cp target/release/wbuild/poc-runtime/poc_runtime.compact.compressed.wasm output
	chain-spec-builder -c output/chainspec.json create -n poc-runtime -i poc-runtime -r ./output/poc_runtime.compact.compressed.wasm -s default
	cat output/chainspec.json | jq '.properties = {}' > output/chainspec.json.tmp
	mv output/chainspec.json.tmp output/chainspec.json
