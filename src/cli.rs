use clap::{ArgEnum, Parser, Subcommand};

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

#[derive(ArgEnum, Clone, Debug)]
pub enum Period {
    #[clap(name = "ALL")]
    ALL,
    #[clap(name = "1D")]
    D1,
    #[clap(name = "5D")]
    D5,
    #[clap(name = "1M")]
    M1,
    #[clap(name = "3M")]
    M3,
    #[clap(name = "6M")]
    M6,
    #[clap(name = "YTD")]
    YTD,
    #[clap(name = "1Y")]
    Y1,
    #[clap(name = "3Y")]
    Y3,
    #[clap(name = "5Y")]
    Y5,
    #[clap(name = "10Y")]
    Y10,
    #[clap(name = "MAX")]
    MAX,
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
        #[clap(
            arg_enum,
            default_value = "ALL",
            help = "Period of time for change",
            short,
            long
        )]
        period: Period,
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
