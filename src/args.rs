use clap::{Args, Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
pub struct DevtoolArgs {
    /// Commands
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Convert Decimal to Hexadecimal.
    Hex(HexArgs),
    /// Convert Hexadecimal to Decimal.
    Decimal(DecimalArgs),
    /// Show limits of integer and floating point types. Provide no arguments to show limits of all
    /// types as such: `devtools limits`.
    Limits(LimitArgs),
}

#[derive(Clone, Debug, Args)]
pub struct HexArgs {
    /// Convert Decimal to Hexadecimal and vice versa.
    #[arg(long, short)]
    pub DecimalToHex: u128,
}

#[derive(Clone, Debug, Args)]
pub struct DecimalArgs {
    #[arg(long, short)]
    pub ToDecimal: String,
}
#[derive(Clone, Debug, Args)]
pub struct LimitArgs {
    /// Show the limits for unsigned integers from 8->128 bits.
    #[arg(long, short)]
    pub uint: bool,
    /// Show the limits for signed integers from 8->128 bits.
    #[arg(long, short)]
    pub int: bool,
    /// Show the limits for floating point digits from 32->64 bits.
    #[arg(long, short)]
    pub float: bool,
}
