use clap::Parser;
use parity_scale_codec::Encode;
use pvq_extension::{extensions_impl, ExtensionsExecutor, InvokeSource};
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to the PolkaVM program to execute
    #[arg(short, long)]
    program: std::path::PathBuf,
}

fn main() {
    let registry = tracing_subscriber::registry();

    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    registry
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .try_init()
        .expect("Failed to initialize tracing");

    let cli = Cli::parse();

    let blob = std::fs::read(&cli.program).expect("Failed to read program");
    let mut executor = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let mut input_data = Vec::new();
    let program_str = cli.program.to_string_lossy();
    if program_str.contains("sum-balance") {
        input_data.extend_from_slice(&0u32.encode());
        input_data.extend_from_slice(&vec![[0u8; 32], [1u8; 32]].encode());
    } else if program_str.contains("total-supply") {
        input_data.extend_from_slice(&0u32.encode());
    } else if program_str.contains("transparent-call") {
        input_data.extend_from_slice(&4071833530116166512u64.encode());
        input_data.extend_from_slice(
            &ExtensionFungiblesFunctions::balance {
                asset: 0,
                who: [1u8; 32],
            }
            .encode(),
        );
    }
    tracing::info!("Input data: {:?}", input_data);
    let res = executor.execute_method(&blob, &input_data, 0).unwrap();

    tracing::info!("Result: {:?}", res);
}

#[derive(Encode)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum ExtensionFungiblesFunctions {
    #[codec(index = 0)]
    total_supply { asset: u32 },
    #[codec(index = 1)]
    balance { asset: u32, who: [u8; 32] },
}

#[extensions_impl]
pub mod extensions {
    #[extensions_impl::impl_struct]
    pub struct ExtensionsImpl;

    #[extensions_impl::extension]
    impl pvq_extension_core::extension::ExtensionCore for ExtensionsImpl {
        type ExtensionId = u64;
        fn has_extension(id: Self::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    #[extensions_impl::extension]
    impl pvq_extension_fungibles::extension::ExtensionFungibles for ExtensionsImpl {
        type AssetId = u32;
        type AccountId = [u8; 32];
        type Balance = u64;
        fn total_supply(_asset: Self::AssetId) -> Self::Balance {
            100
        }
        fn balance(_asset: Self::AssetId, _who: Self::AccountId) -> Self::Balance {
            100
        }
    }
}
