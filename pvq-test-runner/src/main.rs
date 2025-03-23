use clap::Parser;
use tracing_subscriber::prelude::*;

use pvq_test_runner::TestRunner;

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
    let program_str = cli.program.to_string_lossy();

    let input_data = TestRunner::prepare_input_data(&program_str);
    tracing::info!("Input data: {:?}", input_data);

    let mut runner = TestRunner::new();
    let res = runner.execute_program(&blob, &input_data).unwrap();

    tracing::info!("Result: {:?}", res);
}
