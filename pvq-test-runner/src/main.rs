use clap::Parser;
use parity_scale_codec::{Decode, Encode};
use pvq_extension::{impl_extensions, ExtensionsExecutor, InvokeSource};
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
    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);
    let mut input_data = pvq_extension_fungibles::EXTENSION_ID.encode();
    input_data.extend_from_slice(&[2u8]);
    let method1 = FungiblesMethod::Balance {
        asset: 1,
        who: [0u8; 32],
    };
    let method1_encoded = method1.encode();
    input_data.extend_from_slice(&[method1_encoded.len() as u8]);
    let method2 = FungiblesMethod::Balance {
        asset: 1,
        who: [1u8; 32],
    };
    input_data.extend_from_slice(&method1_encoded);
    input_data.extend_from_slice(&method2.encode());
    tracing::info!("Input data: {:?}", input_data);
    let res = executor.execute_method(&blob, &input_data).unwrap();

    tracing::info!("Result: {:?}", res);
}

pub struct ExtensionImpl;

impl pvq_extension_core::Config for ExtensionImpl {
    type ExtensionId = u64;
}

impl pvq_extension_fungibles::Config for ExtensionImpl {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
}

impl_extensions! {
    impl pvq_extension_core::ExtensionCore for ExtensionImpl {
        type Config = ExtensionImpl;
        fn has_extension(id: <Self::Config as pvq_extension_core::Config>::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    impl pvq_extension_fungibles::ExtensionFungibles for ExtensionImpl {
        type Config = ExtensionImpl;
        #[allow(unused_variables)]
        fn balance(
            asset: <Self::Config as pvq_extension_fungibles::Config>::AssetId,
            who: <Self::Config as pvq_extension_fungibles::Config>::AccountId,
        ) -> <Self::Config as pvq_extension_fungibles::Config>::Balance {
            100
        }
        #[allow(unused_variables)]
        fn total_supply(asset: <Self::Config as pvq_extension_fungibles::Config>::AssetId) -> <Self::Config as pvq_extension_fungibles::Config>::Balance {
           200
        }
    }
}
#[derive(Encode, Decode)]
enum CoreMethod {
    HasExtension { id: u64 },
}

#[derive(Encode, Decode)]
enum FungiblesMethod {
    TotalSupply { asset: u64 },
    Balance { asset: u64, who: [u8; 32] },
}
