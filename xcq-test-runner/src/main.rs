use clap::Parser;
use polkavm::{Caller, Config, Linker};
use tracing_subscriber::prelude::*;
use xcq_executor::{XcqExecutor, XcqExecutorContext};

struct HostFunctions;

#[derive(Clone, Copy)]
#[repr(C)]
struct GuestArgs {
    arg0: u32,
    arg1: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct GuestReturn {
    data0: u64,
    data1: u64,
}

impl XcqExecutorContext for HostFunctions {
    fn register_host_functions<T>(&mut self, linker: &mut Linker<T>) {
        linker
            .func_wrap(
                "host_call_impl",
                move |mut caller: Caller<_>, args_ptr: u32, out_ptr: u32| {
                    let args_ptr = args_ptr as *const GuestArgs;
                    let args_size = core::mem::size_of::<GuestArgs>();
                    // First we read the args from the guest memory
                    let args_in_bytes = caller.read_memory_into_vec(args_ptr as u32, args_size as u32).unwrap();
                    let args: GuestArgs = unsafe { std::ptr::read(args_in_bytes.as_ptr() as *const GuestArgs) };
                    println!("host_call: arg0: {:?}", args.arg0);
                    let res = GuestReturn {
                        data0: (args.arg0 + 1) as u64,
                        data1: args.arg1 as u64,
                    };
                    let res_bytes = unsafe {
                        std::slice::from_raw_parts(
                            &res as *const GuestReturn as *const u8,
                            core::mem::size_of::<GuestReturn>(),
                        )
                    };
                    caller.write_memory(out_ptr, res_bytes).unwrap();
                },
            )
            .unwrap();
    }
}

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

    let config = Config::from_env().unwrap();

    let mut executor: XcqExecutor<HostFunctions> = XcqExecutor::new(config, HostFunctions);
    let res = executor.execute(&raw_blob[..], &[0u8]).unwrap();
    tracing::info!("Result: {:?}", res);

    let res = executor.execute(&raw_blob[..], &[1u8, 40u8]).unwrap();
    tracing::info!("Result: {:?}", res);
}
