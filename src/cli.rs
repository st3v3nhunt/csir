use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about = "CLI Stock Info Retriever")]
#[clap(author = "st3v3nhunt", version)]
pub struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
    /// Type of request to make for symbol
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Real-time price of symbol(s)
    Price {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "", max_values = 1)]
        symbols: Vec<String>,
    },
    /// Percentage price change of symbol(s)
    PriceChange {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "")]
        symbols: Vec<String>,
    },
    /// Full quote for symbol(s)
    Quote {
        /// Symbol(s) to retrieve info about
        #[clap(forbid_empty_values = true, default_value = "", max_values = 1)]
        symbols: Vec<String>,
    },
}
