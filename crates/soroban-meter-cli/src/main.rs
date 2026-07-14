mod output;
mod simulate;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "soroban-meter", version = "0.1.0", author = "Gideon Bature")]
#[command(about = "Resource profiling for Soroban smart contracts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Profile a deployed function against testnet/mainnet
    Profile {
        #[arg(long)]
        contract: String,

        #[arg(long)]
        function: String,

        #[arg(long)]
        args: Option<String>,

        #[arg(long, default_value = "testnet")]
        network: String,

        #[arg(long, default_value = "table")]
        output: String,

        #[arg(long)]
        baseline: Option<String>,

        #[arg(long)]
        fail_on_regression: Option<f64>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Profile {
            contract,
            function,
            args,
            network,
            output,
            baseline,
            fail_on_regression,
        } => {
            let result =
                simulate::simulate_transaction(contract, function, args.as_deref(), network)
                    .await?;

            if let Some(baseline_path) = &baseline {
                output::check_regression(&result, baseline_path, fail_on_regression)?;
            }

            if output == "json" {
                output::print_json(&result);
            } else {
                output::print_table(&result, contract, function, network);
            }
        }
    }

    Ok(())
}
