use clap::Parser;
use parity_scale_codec::{Decode, Encode};
use tracing_subscriber::prelude::*;
use xcq_extension::{impl_extensions, ExtensionsExecutor, Guest, Input, InvokeSource, Method};

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
    let guest = GuestImpl { program: blob.to_vec() };
    let mut input_data = xcq_extension_fungibles::EXTENSION_ID.encode();
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
    input_data.extend_from_slice(&xcq_extension_fungibles::EXTENSION_ID.encode());
    let method3 = FungiblesMethod::TotalSupply { asset: 1 };
    let method3_encoded = method3.encode();
    input_data.extend_from_slice(&[method3_encoded.len() as u8]);
    input_data.extend_from_slice(&method3_encoded);
    tracing::info!("Input data: {:?}", input_data);
    let input = InputImpl {
        method: "main".to_string(),
        args: input_data,
    };
    let res = executor.execute_method(guest, input).unwrap();

    tracing::info!("Result: {:?}", res);
}

pub struct ExtensionImpl;

impl xcq_extension_core::Config for ExtensionImpl {
    type ExtensionId = u64;
}

impl xcq_extension_fungibles::Config for ExtensionImpl {
    type AssetId = u32;
    type AccountId = [u8; 32];
    type Balance = u64;
}

impl_extensions! {
    impl xcq_extension_core::ExtensionCore for ExtensionImpl {
        type Config = ExtensionImpl;
        fn has_extension(id: <Self::Config as xcq_extension_core::Config>::ExtensionId) -> bool {
            matches!(id, 0 | 1)
        }
    }

    impl xcq_extension_fungibles::ExtensionFungibles for ExtensionImpl {
        type Config = ExtensionImpl;
        #[allow(unused_variables)]
        fn balance(
            asset: <Self::Config as xcq_extension_fungibles::Config>::AssetId,
            who: <Self::Config as xcq_extension_fungibles::Config>::AccountId,
        ) -> <Self::Config as xcq_extension_fungibles::Config>::Balance {
            100
        }
        #[allow(unused_variables)]
        fn total_supply(asset: <Self::Config as xcq_extension_fungibles::Config>::AssetId) -> <Self::Config as xcq_extension_fungibles::Config>::Balance {
           200
        }
    }
}
// guest impls
pub struct GuestImpl {
    pub program: Vec<u8>,
}

impl Guest for GuestImpl {
    fn program(&self) -> &[u8] {
        &self.program
    }
}

pub struct InputImpl {
    pub method: Method,
    pub args: Vec<u8>,
}

impl Input for InputImpl {
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn args(&self) -> &[u8] {
        &self.args
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
