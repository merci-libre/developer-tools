use args::*;
use clap::Parser;
use dectohex::convert_to_hex;

mod args;
mod dectohex;
/* prints all of the max integers for each
* data type. (1-overflow point.)*/
fn signed() {
    println!(
        "Signed Integers Maximums:\n8-bit:   {}\n16-bit:  {} \n32-bit:  {} \n64-bit:  {} \n128-bit: {}",
        std::i8::MAX,
        std::i16::MAX,
        std::i32::MAX,
        std::i64::MAX,
        std::i128::MAX
    );
    println!(
        "\nSigned Integers Minimums:\n8-bit:   {}\n16-bit:  {} \n32-bit:  {} \n64-bit:  {} \n128-bit: {}",
        std::i8::MIN,
        std::i16::MIN,
        std::i32::MIN,
        std::i64::MIN,
        std::i128::MIN
    );
}
fn unsigned(no_args: u8) {
    if no_args == 1 {
        println!("")
    }
    println!(
        "Unsigned Integers Maximums: \n8-bit:   {}\n16-bit:  {}\n32-bit:  {}\n64-bit:  {}\n128-bit: {}",
        std::u8::MAX,
        std::u16::MAX,
        std::u32::MAX,
        std::u64::MAX,
        std::u128::MAX
        );
    println!(
            "\nUnsigned Integers Minimums: \n8-bit:   {}\n16-bit:  {}\n32-bit:  {}\n64-bit:  {}\n128-bit: {}",
            std::u8::MIN,
            std::u16::MIN,
            std::u32::MIN,
            std::u64::MIN,
            std::u128::MIN
        );
}
fn floatingpoint(no_args: u8) {
    if no_args == 1 {
        println!("")
    }
    println!(
        "Floating Point Maxes:\n\n32-bit: \n{:.08}\n\n64-bit: \n{:.08}",
        std::f32::MAX,
        std::f64::MAX
    );
    println!(
        "\nFloating Point Mins:\n\n32-bit: \n{:.08}\n\n64-bit: \n{:.08}",
        std::f32::MIN,
        std::f64::MIN
    );
}
fn main() {
    let args = DevtoolArgs::parse();
    let command = args.commands;
    match command {
        Commands::Hex(HexArgs) => {
            let input = HexArgs.DecimalToHex;
            println!("{}", convert_to_hex(input));
        }
        Commands::Limits(LimitArgs) => {
            if LimitArgs.uint {
                println!("Developer tools by westwardfishdme.\n( https://github.com/westwardfishdme | https://westwardfishd.me )\n");
                unsigned(0);
            }
            if LimitArgs.int {
                println!("Developer tools by westwardfishdme.\n( https://github.com/westwardfishdme | https://westwardfishd.me )\n");
                signed();
            }
            if LimitArgs.float {
                println!("Developer tools by westwardfishdme.\n( https://github.com/westwardfishdme | https://westwardfishd.me )\n");
                floatingpoint(0);
            }
            if !LimitArgs.uint && !LimitArgs.int && !LimitArgs.float {
                println!("Developer tools by westwardfishdme.\n( https://github.com/westwardfishdme | https://westwardfishd.me )\n");
                signed();
                unsigned(1);
                floatingpoint(1);
            }
        }
        _ => todo!(),
    }
}
