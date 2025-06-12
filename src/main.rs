use args::*;
use clap::Parser;
use dectohex::{
    convert_from_hex_file, convert_hex_file, convert_to_dec_from_hex, convert_to_hex,
    rotate_char_to_hex,
};
use std::{fmt::format, str::FromStr};

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
fn convertbytes(vec: Vec<u8>, prefix: bool, spaces: bool, add_machine_prefix: bool, rotate: i64) {
    let mut string: String = String::new();
    for i in vec {
        let mut output = String::new();
        match rotate {
            0 => output = convert_to_hex(i.into()),
            _ => output = rotate_char_to_hex(rotate, i.into()),
        }
        let mut before_return: String = String::new();

        match add_machine_prefix {
            true => {
                before_return = format!("\\x{} ", output.strip_prefix("0x").unwrap());
            }
            false => (),
        }
        if !add_machine_prefix {
            match prefix {
                true => before_return = format!("{} ", output.strip_prefix("0x").unwrap()),
                false => before_return = format!("{} ", output),
            }
        }
        string.push_str(before_return.as_str());
    }

    match spaces {
        true => string = string.replace(" ", ""),
        false => (),
    }

    println!("{}", string);
    eprint!("\n");
}
fn main() {
    let args = DevtoolArgs::parse();
    let command = args.commands;
    eprintln!("Developer tools by westwardfishdme.\n( https://github.com/westwardfishdme | https://westwardfishd.me )\n");
    match command {
        Commands::Hex(HexArgs) => {
            let input = HexArgs.decimal_to_hex;
            let mut raw = HexArgs.raw;
            if HexArgs.rotate != 0 {
                raw = true;
            }

            //convert a raw string if option is given, does not try to interpret otherwise.
            if raw {
                convertbytes(
                    input.into_bytes(),
                    HexArgs.no_prefix,
                    HexArgs.spaces,
                    HexArgs.machine,
                    HexArgs.rotate,
                );
            } else {
                match input.parse::<u128>() {
                    Ok(n) => {
                        let mut string: String = convert_to_hex(n);
                        if HexArgs.no_prefix {
                            string = string.strip_prefix("0x").unwrap().to_string();
                        }

                        println!("{}", string)
                    }
                    Err(_) => convertbytes(
                        input.into_bytes(),
                        HexArgs.no_prefix,
                        HexArgs.spaces,
                        HexArgs.machine,
                        HexArgs.rotate,
                    ),
                }
            }

            if !HexArgs.convert_file.is_empty() {
                convert_hex_file(
                    HexArgs.convert_file,
                    !HexArgs.output.is_empty(),
                    HexArgs.carriage_return,
                    HexArgs.output,
                )
                .unwrap();
            }
        }
        Commands::Decimal(DecimalArgs) => {
            let input = DecimalArgs.to_decimal;
            if input.contains("0x") {
                input.strip_prefix("0x").unwrap();
            }
            if !input.is_empty() {
                let decimal = convert_to_dec_from_hex(input).unwrap();
                println!("{}", decimal);
            }

            if !DecimalArgs.convert_file.is_empty() {
                convert_from_hex_file(
                    DecimalArgs.convert_file,
                    !DecimalArgs.output.is_empty(),
                    DecimalArgs.output,
                )
                .unwrap();
            }
        }
        Commands::Limits(LimitArgs) => {
            if LimitArgs.uint {
                unsigned(0);
            }
            if LimitArgs.int {
                signed();
            }
            if LimitArgs.float {
                floatingpoint(0);
            }
            if !LimitArgs.uint && !LimitArgs.int && !LimitArgs.float {
                signed();
                unsigned(1);
                floatingpoint(1);
            }
        }
    }
}
