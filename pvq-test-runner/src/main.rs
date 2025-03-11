use clap::Parser;
use parity_scale_codec::{Decode, Encode};
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

    let blob = std::fs::read(cli.program).expect("Failed to read program");
    let mut executor = ExtensionsExecutor::<extensions::Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let mut input_data = pvq_extension_core::extension::extension_id().encode();
    input_data.extend_from_slice(&[2u8]);
    let method1 = pvq_extension_fungibles::extension::Functions::<extensions::ExtensionsImpl>::balance {
        asset: 1,
        who: [0u8; 32],
    }
    .encode();
    let method1_encoded = method1.encode();
    input_data.extend_from_slice(&[method1_encoded.len() as u8]);
    let method2 = pvq_extension_fungibles::extension::Functions::<extensions::ExtensionsImpl>::balance {
        asset: 1,
        who: [1u8; 32],
    }
    .encode();
    input_data.extend_from_slice(&method1_encoded);
    input_data.extend_from_slice(&method2.encode());
    tracing::info!("Input data: {:?}", input_data);
    let res = executor.execute_method(&blob, &input_data, 0).unwrap();

    tracing::info!("Result: {:?}", res);
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
        fn total_supply(asset: Self::AssetId) -> Self::Balance {
            100
        }
        fn balance(asset: Self::AssetId, who: Self::AccountId) -> Self::Balance {
            100
        }
    }
}
