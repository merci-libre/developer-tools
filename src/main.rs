use crate::dectohex::{
    convert_from_hex_file, convert_hex_file, convert_to_dec_from_hex, convert_to_hex,
};
use crate::typeinfo::*;
use args::*;
use chmod::chmod_perms_calculator;
use clap::Parser;
use std::error::Error;

mod args;
mod chmod;
mod dectohex;
mod typeinfo;
/* prints all of the max integers for each
* data type. (1-overflow point.)*/
fn main() -> Result<(), Box<dyn Error>> {
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
            Ok(())
        }
        Commands::Decimal(DecimalArgs) => {
            let mut input = DecimalArgs.to_decimal;
            if input.contains("0x") {
                input = input.replace("0x", "");
            }
            if !input.is_empty() {
                let decimal = convert_to_dec_from_hex(input)?;
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
            Ok(())
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
            Ok(())
        }
        Commands::Sizes(SizeArgs) => {
            /*TODO:
             * - Create a way to match boolean values to select a mode for each type without
             * actually passing in each type.
             *
             *
             *
             * */
            sizes();
            Ok(())
        }
        Commands::Chmod(ChmodArgs) => {
            let test_string = ChmodArgs.calculate;
            let (
                value,          /*String*/
                interpretation, /*String*/
                permissions,    /*bTreemap*/
            ) = chmod_perms_calculator(test_string);

            println!("Permissions String:\n{interpretation}\nChmod Octal Value: {value}");

            for (class, perms) in permissions {
                match class.as_str() {
                    "owner" => {
                        eprintln!(
                    "\nThe owner has the following permissions:\nread: {}\nwrite: {}\nexecute: {}",
                    perms.readperms, perms.writeperms, perms.executeperms
                );
                    }
                    "group" => {
                        eprintln!(
                    "\nAll members of a group have the following permissions:\nread: {}\nwrite: {}\nexecute: {}",
                    perms.readperms, perms.writeperms, perms.executeperms
                );
                    }
                    "public" => {
                        eprintln!(
                    "\nAll users have the following permissions:\nread: {}\nwrite: {}\nexecute: {}",
                    perms.readperms, perms.writeperms, perms.executeperms
                );
                    }
                    _ => (),
                }
            }
            Ok(())
        }
    }
}
