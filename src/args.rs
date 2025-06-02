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
    /// Converts an entire file to Hexadecimal. STDOUT is hexadecimal.
    #[arg(long, short, default_value = "")]
    pub convert_file: String,
    /// When converting a plaintext file to hex, keep the carriage return values (0x10).
    #[arg(long, short = 'n')]
    pub carriage_return: bool,
    /// Outputs relative information to a text file (hex, decimal values, and original text).
    #[arg(long, short, default_value = "")]
    pub output: String,
    /// Converts a single Decimal to Hexadecimal. If the input is a string, it converts the string
    ///
    /// to bytes and then returns the hexadecimal values of the string.
    ///
    /// If a single number is given, it converts that number to it's represented hexadecimal value.
    #[arg(long, short, default_value = "")]
    pub decimal_to_hex: String,
    /// always convert the inputted string to raw bytes.
    #[arg(long)]
    pub raw: bool,
}

#[derive(Clone, Debug, Args)]
pub struct DecimalArgs {
    /// Converts a file containing hexadecimal to decimal values
    #[arg(long, short, default_value = "")]
    pub convert_file: String,
    /// Outputs relative information to a text file (hex, decimal values, and original text).
    #[arg(long, short, default_value = "")]
    pub output: String,
    /// Converts hexadecimal to decimal values.
    #[arg(long, short, default_value = "")]
    pub to_decimal: String,
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
