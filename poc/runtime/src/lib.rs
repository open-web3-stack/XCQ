#![cfg_attr(not(feature = "std"), no_std)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use polkadot_sdk_frame::{
	deps::frame_support::{
		runtime,
		weights::{FixedFee, NoFee},
	},
	prelude::*,
	runtime::{
		apis::{
			self, impl_runtime_apis, ApplyExtrinsicResult, CheckInherentsResult,
			ExtrinsicInclusionMode, OpaqueMetadata,
		},
		prelude::*,
	},
};

/// The runtime version.
#[runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("xcq-poc"),
	impl_name: create_runtime_str!("xcq-poc"),
	authoring_version: 1,
	spec_version: 0,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The signed extensions that are added to the runtime.
type SignedExtra = (
	// Checks that the sender is not the zero address.
	frame_system::CheckNonZeroSender<Runtime>,
	// Checks that the runtime version is correct.
	frame_system::CheckSpecVersion<Runtime>,
	// Checks that the transaction version is correct.
	frame_system::CheckTxVersion<Runtime>,
	// Checks that the genesis hash is correct.
	frame_system::CheckGenesis<Runtime>,
	// Checks that the era is valid.
	frame_system::CheckEra<Runtime>,
	// Checks that the nonce is valid.
	frame_system::CheckNonce<Runtime>,
	// Checks that the weight is valid.
	frame_system::CheckWeight<Runtime>,
	// Ensures that the sender has enough funds to pay for the transaction
	// and deducts the fee from the sender's account.
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

// Composes the runtime by adding all the used pallets and deriving necessary types.
#[runtime]
mod runtime {
	/// The main runtime type.
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask
	)]
	pub struct Runtime;

	/// Mandatory system pallet that should always be included in a FRAME runtime.
	#[runtime::pallet_index(0)]
	pub type System = frame_system;

	/// Provides a way for consensus systems to set and check the onchain time.
	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp;

	/// Provides the ability to keep track of balances.
	#[runtime::pallet_index(2)]
	pub type Balances = pallet_balances;

	/// Provides a way to execute privileged functions.
	#[runtime::pallet_index(3)]
	pub type Sudo = pallet_sudo;

	/// Provides the ability to charge for extrinsic execution.
	#[runtime::pallet_index(4)]
	pub type TransactionPayment = pallet_transaction_payment;
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

/// Implements the types required for the system pallet.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type Version = Version;
	// Use the account data from the balances pallet
	type AccountData = pallet_balances::AccountData<<Runtime as pallet_balances::Config>::Balance>;
}

// Implements the types required for the balances pallet.
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Runtime {
	type AccountStore = System;
}

// Implements the types required for the sudo pallet.
#[derive_impl(pallet_sudo::config_preludes::TestDefaultConfig)]
impl pallet_sudo::Config for Runtime {}

// Implements the types required for the sudo pallet.
#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Runtime {}

// Implements the types required for the transaction payment pallet.
#[derive_impl(pallet_transaction_payment::config_preludes::TestDefaultConfig)]
impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
	// Setting fee as independent of the weight of the extrinsic for demo purposes
	type WeightToFee = NoFee<<Self as pallet_balances::Config>::Balance>;
	// Setting fee as fixed for any length of the call data for demo purposes
	type LengthToFee = FixedFee<1, <Self as pallet_balances::Config>::Balance>;
}

type Block = runtime::types_common::BlockOf<Runtime, SignedExtra>;
type Header = HeaderFor<Runtime>;

type RuntimeExecutive =
	Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};

impl_runtime_apis! {
	impl apis::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			RuntimeExecutive::execute_block(block)
		}

		fn initialize_block(header: &Header) -> ExtrinsicInclusionMode {
			RuntimeExecutive::initialize_block(header)
		}
	}
	impl apis::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl apis::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: ExtrinsicFor<Runtime>) -> ApplyExtrinsicResult {
			RuntimeExecutive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> HeaderFor<Runtime> {
			RuntimeExecutive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<ExtrinsicFor<Runtime>> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: InherentData,
		) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl apis::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: ExtrinsicFor<Runtime>,
			block_hash: <Runtime as frame_system::Config>::Hash,
		) -> TransactionValidity {
			RuntimeExecutive::validate_transaction(source, tx, block_hash)
		}
	}

	impl apis::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &HeaderFor<Runtime>) {
			RuntimeExecutive::offchain_worker(header)
		}
	}

	impl apis::SessionKeys<Block> for Runtime {
		fn generate_session_keys(_seed: Option<Vec<u8>>) -> Vec<u8> {
			Default::default()
		}

		fn decode_session_keys(
			_encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, apis::KeyTypeId)>> {
			Default::default()
		}
	}

	impl apis::AccountNonceApi<Block, interface::AccountId, interface::Nonce> for Runtime {
		fn account_nonce(account: interface::AccountId) -> interface::Nonce {
			System::account_nonce(account)
		}
	}
}

pub mod interface {
	use super::Runtime;
	use polkadot_sdk_frame::deps::frame_system;

	pub type Block = super::Block;
	pub use polkadot_sdk_frame::runtime::types_common::OpaqueBlock;
	pub type AccountId = <Runtime as frame_system::Config>::AccountId;
	pub type Nonce = <Runtime as frame_system::Config>::Nonce;
	pub type Hash = <Runtime as frame_system::Config>::Hash;
	pub type Balance = <Runtime as pallet_balances::Config>::Balance;
	pub type MinimumBalance = <Runtime as pallet_balances::Config>::ExistentialDeposit;
}
