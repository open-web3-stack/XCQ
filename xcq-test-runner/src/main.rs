use clap::Parser;
use parity_scale_codec::{Decode, Encode};
use tracing_subscriber::prelude::*;
use xcq_extension::{impl_extensions, ExtensionId, ExtensionsExecutor, Guest, Input, InvokeSource, Method};

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

    let raw_blob = std::fs::read(cli.program).expect("Failed to read program");

    let mut executor = ExtensionsExecutor::<Extensions, ()>::new(InvokeSource::RuntimeAPI);

    let guest = GuestImpl {
        program: raw_blob.to_vec(),
    };
    let method = CoreMethod::HasExtension { id: 0 };
    let mut input_data =
        <xcq_extension_core::decl_extension_for_extension_core::Call<ExtensionImpl> as ExtensionId>::EXTENSION_ID
            .encode();
    input_data.extend_from_slice(&method.encode());
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
            0
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
