/**
*
* #### main.rs ####
*
* A small command-line interface to explore de Bruijn graph.
*
* While this repository is mainly aimed at being a library, we propose a small
* interface to practically interact with the results of the paper. Specifically,
* - dbg_cycles count [PARAMS], is used to count the cycles
* - dbg_cycles enum  [PARAMS], is used to enumerate them the cycles
* - dbg_cycles conjecture    , is used to assess the conjecture on the (few) values that are tractable
*
* The binary alphabet is consider by default. If no length parameter is set,
* then the research will be carried for all cycles
*
**/
use clap::{Parser, Subcommand};
use colored::Colorize;
use dbg_cycles::count::count_cycles_only_enum;
use dbg_cycles::count::{count_cycles_with_formula, Count};
use dbg_cycles::r#enum::{enum_cycles_bounded_length, enum_cycles_fixed_length};

//
// We rely on the clap crate for parsing arguments and displaying help messages.
//

#[derive(Parser, Debug)]
#[command(name = "dbg_cycles")]
#[command(author = "Léo Ackermann")]
#[command(
    about = r#"
╔╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╗
╠╬╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╬╣
╠╣         _ ____   ____                   _              ╠╣
╠╣      __| | __ ) / ___|   ___ _   _  ___| | ___  ___    ╠╣
╠╣     / _` |  _ \| |  _   / __| | | |/ __| |/ _ \/ __|   ╠╣
╠╣    | (_| | |_) | |_| | | (__| |_| | (__| |  __/\__ \   ╠╣
╠╣     \__,_|____/ \____|  \___|\__, |\___|_|\___||___/   ╠╣
╠╣                              |___/                     ╠╣
╠╬╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╦╬╣
╚╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╩╝
                                            © Léo Ackermann

    A small CLI tool to count and enumerate simple cycles
                  in the de Bruijn graph
"#,
    version = "1.0"
)]

struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Count simple cycles of the de Bruijn graph.
    Count {
        /// Order of the de Bruijn graph
        #[arg(short = 'k', long)]
        order: usize,
        /// Length of the cycles
        #[arg(short = 'l', long, default_value_t = 0)]
        length: usize,
        /// Size of the alphabet
        #[arg(short = 's', long, default_value_t = 2)]
        sigma: u8,
    },

    /// Enumerate simple cycles of the de Bruijn graph (of length no larger than the order)
    Enum {
        /// Order of the de Bruijn graph
        #[arg(short = 'k', long)]
        order: usize,
        /// Length of the cycles
        #[arg(short = 'l', long, default_value_t = 0)]
        length: usize,
        /// Size of the alphabet
        #[arg(short = 's', long, default_value_t = 2)]
        sigma: u8,
    },

    /// Test the conjecture by comparing the result obtain with enumeration
    Conjecture,
}

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
            cli_enum(*length, *order, *sigma);
        }
        Commands::Conjecture => cli_test_conjecture_plusthree(),
    }
}

// Returns the number of simple cycles in the dbg. we highlight where this
// number comes from (proved/conjectured formula, or enumeration based)
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
            Count::FromEnum(x) => {
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

// For testing the conjecture, we compare the proposed number to the one
// computed using enumeration. We scan the (k, sigma) space diagonally so that
// (A) we are not restricted to near-border regimes, that could be specific (B)
// we start by the less demanding computations.
fn cli_test_conjecture_plusthree() {
    for y in 0.. {
        for x in 0..=y {
            // Run comparison on sigma-order, order
            let sigma = y - x + 2;
            let order = x + 2;
            print!("(s={}, k={})\t", sigma, order);
            if let Count::FromConjecturedFormula(conjecture) =
                count_cycles_with_formula(order + 3, order, sigma as u8, true)
            {
                let count_enum = count_cycles_only_enum(order + 3, order, sigma as u8)
                    .to_option()
                    .unwrap();
                let comparison = if conjecture == count_enum {
                    "===".green()
                } else {
                    "=/=".red()
                };
                println!("{}\t {}\t {}", conjecture, comparison, count_enum);
            }
        }
    }
}

// Pretty print the simple cycles in the dbg. When no length is given, the
// cycles are generated using the bounded-length iterator on Lyndon words, and
// then sorted/grouped by cycle size. We highlight where this number comes from
// (proved/conjectured formula, or enumeration based)
fn cli_enum(length: usize, order: usize, sigma: u8) {
    if length != 0 {
        let cycles = enum_cycles_fixed_length(length, order, sigma);

        println!(
            "The {} simple cycles of length {} in dBG({}, {}) are",
            cycles.len(),
            length,
            order,
            sigma
        );
        for (_, cycle) in cycles.iter().enumerate() {
            print!("  ");
            for (j, word) in cycle.iter().enumerate() {
                if j != 0 {
                    print!(" --> ");
                }
                for (k, letter) in word.iter().enumerate() {
                    if k != 0 {
                        print!(".");
                    }
                    print!("{}", letter);
                }
            }
            println!();
        }
    } else {
        let max_cycle_length = usize::pow(sigma as usize, order as u32);
        let mut cycles = enum_cycles_bounded_length(max_cycle_length, order, sigma);
        cycles.sort_by_key(|x| (x.len(), x.clone()));
        let mut current_len = 0;
        println!("In the de Bruijn graph dBG({}, {})...", order, sigma);
        for (_, cycle) in cycles.iter().enumerate() {
            if current_len != cycle.len() {
                current_len = cycle.len();
                println!("\n..the simple cycles of length {}", current_len);
            }
            print!("  ");
            for (j, word) in cycle.iter().enumerate() {
                if j != 0 {
                    print!(" --> ");
                }
                for (k, letter) in word.iter().enumerate() {
                    if k != 0 {
                        print!(".");
                    }
                    print!("{}", letter);
                }
            }
            println!();
        }
    }
}
