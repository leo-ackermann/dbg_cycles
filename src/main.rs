use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dbg_cycles")]
#[command(about = "CLI tool to count and enumerate simple cycles in the de Bruijn graph", version = "1.0")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Count simple cycles of the de Bruijn graph
    Count {
        /// Order of the de Bruijn graph
        #[arg(short, long)]
        order: u8,
        /// Length of the cycles
        #[arg(short, long)]
        length: u8,
        /// Size of the alphabet
        #[arg(short, long, default_value_t = 2)]
        sigma: u8,
        /// If enable, do not relies on formulas for computing the number of non-<order>-Lyndon words
        #[arg(long)]
        brute_force: bool,
    },

     /// Enumerate simple cycles of the de Bruijn graph (of length no larger than the order)
    Enum {
        /// Order of the de Bruijn graph
        #[arg(short, long)]
        order: u8,
        /// Length of the cycles
        #[arg(short, long)]
        length: u8,
        /// Size of the alphabet
        #[arg(short, long, default_value_t = 2)]
        sigma: u8,
    },
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Count {
            order,
            length,
            sigma,
            brute_force,
        } => {
            println!(
                "Running dbg_cycles count with order={}, length={}, sigma={}, brute_force={}",
                order, length, sigma, brute_force
            );
            // TODO: Call
        }
        Commands::Enum {
            order,
            length,
            sigma,
        } => {
            println!(
                "Running dbg_cycles enum with order={}, length={}, sigma={}",
                order, length, sigma
            );
            // TODO: Call
        }
    }
}
