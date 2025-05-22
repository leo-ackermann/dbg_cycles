use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dbg_cycles")]
#[command(
    about = "CLI tool to count and enumerate simple cycles in the de Bruijn graph",
    version = "1.0"
)]
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
        order: usize,
        /// Length of the cycles
        #[arg(short, long, default_value_t = 0)]
        length: usize,
        /// Size of the alphabet
        #[arg(short, long, default_value_t = 2)]
        sigma: u8,
    },

    /// Enumerate simple cycles of the de Bruijn graph (of length no larger than the order)
    Enum {
        /// Order of the de Bruijn graph
        #[arg(short, long)]
        order: usize,
        /// Length of the cycles
        #[arg(short, long)]
        length: usize,
        /// Size of the alphabet
        #[arg(short, long, default_value_t = 2)]
        sigma: usize,
    },
}

use colored::Colorize;

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Count {
            order,
            length,
            sigma,
        } => {
            cli_count(*length, *order, *sigma);
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

use dbg_cycles::formulae::{count_cycles_with_formula, Count};

fn cli_count(length: usize, order: usize, sigma: u8) {
    if length != 0 {
        let count = count_cycles_with_formula(length, order, sigma, false);
        let mut answer = 0;
        let mut status = "dummy".purple();
        match count {
            Count::FromProvedFormula(x) => {
                status = "proved".green();
                answer = x;
            }
            Count::FromConjecturedFormula(x) => {
                status = "conjectured".yellow();
                answer = x;
            }
            Count::FromConjecturedFormula(x) => {
                status = "computed".blue();
                answer = x;
            }
            _ => (),
        }
        println!("There are {} simple cycles of length {} in the deBruijn graph of order {} over the [0..{}) alphabet ({})",
                 answer,
                 length,
                 order,
                 sigma,
                 status,
        )
    } else {
        println!("Within dBG({}, {}), one can find...\n", order, sigma);
        for l in 1..=usize::pow(sigma as usize, order as u32) {
            let count = count_cycles_with_formula(l, order, sigma, false);
            let mut answer = 0;
            let mut status = "dummy".purple();
            match count {
                Count::FromProvedFormula(x) => {
                    status = "proved".green();
                    answer = x;
                }
                Count::FromConjecturedFormula(x) => {
                    status = "conjectured".yellow();
                    answer = x;
                }
                Count::FromEnum(x) => {
                    status = "computed".blue();
                    answer = x;
                }
                _ => (),
            }
            println!(
                "...simple cycles of length {}:\t{}\t({})",
                l, answer, status,
            );
        }
    }
}

fn cli_test_conjecture() {}
