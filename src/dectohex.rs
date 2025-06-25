use std::{
    fs::{self, File},
    io::Write,
    num::ParseIntError,
    process::exit,
};
pub fn convert_to_hex(number: u128) -> String {
    let result = format!("{:#x}", number);
    return result;
}
pub fn convert_to_dec_from_hex(hex: String) -> Result<u128, ParseIntError> {
    let decimal = u128::from_str_radix(hex.as_str(), 16)?;
    Ok(decimal)
}
fn check_readability(path: &String) {
    match fs::read(path.clone()) {
        Ok(_v) => (),
        Err(e) => {
            eprintln!("An error occured trying to read the file:\n{e}");
            exit(1);
        }
    }
}
pub fn rotate_char_to_hex(rotation: i64, value: u8) -> String {
    let mut rotation_result = value as i64 + rotation;
    if rotation_result < 0 {
        rotation_result = 255 - (rotation_result.abs() % 255);
    }
    if rotation_result > 255 {
        rotation_result = rotation_result % 255;
    }
    let result = format!("{:#x}", rotation_result);

    return result;
}

fn vec_to_string(vec: Vec<u8>) -> String {
    let s = match String::from_utf8(vec) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("how the hell did you do this?\n{e}\ntell the dev: line 29 dectohex.rs");
            exit(1)
        }
    };
    return s.to_string();
}

pub fn convert_from_hex_file(path: String, out: bool, mut new_path: String) -> Result<(), ()> {
    check_readability(&path);
    let mut contents: Vec<u8> = fs::read(path.clone()).unwrap();
    let mut output_vec: Vec<u8> = Vec::new();
    contents.retain(|value| *value != 32); // drop all empty 'space' values.
    let mut count = 0;
    /* create a mutable string that gets dropped once conditions are met to convert a hex number to
    decimal. */
    let mut temp_string: String = String::new();
    // where the actual conversion happens.
    for i in contents.clone() {
        temp_string.push(i.into());
        if count % 2 == 1 {
            // every 2 characters
            match convert_to_dec_from_hex(temp_string.clone()) {
                Ok(v) => {
                    eprint!("{} ", v); // print out the character
                    output_vec.push(v as u8); /* since the max value is 'ff' values >255 should never be reached */
                }
                Err(_) => (), // any unrecognizable letter/char just gets ignored.
            }
            temp_string.clear();
        }
        count += 1;
    }
    eprint!("\n");

    eprintln!("Plaintext: {}", vec_to_string(output_vec.clone()));

    // handle file output if user defines.
    if out {
        new_path.push_str(".out");
        match fs::exists(&new_path) {
            Ok(true) => (),
            Ok(false) => (),
            Err(e) => {
                eprintln!("An error occurred.\n{e}");
                exit(1);
            }
        }

        let mut out_file = File::create(new_path.clone()).unwrap();
        //create a newline
        output_vec.push(10);
        out_file.write_all(b"hex values: ").unwrap();
        out_file
            .write_all(&contents)
            .expect("something went wrong.\ntell the dev: line 82 dectohex.rs");

        out_file.write_all(b"decimal values: ").unwrap();
        out_file
            .write_fmt(format_args!("{:?}\n", &output_vec))
            .expect("something went wrong writing the file.\ntell the dev: line 87 dectohex.rs");
        out_file.write_all(b"plaintext: ").unwrap();
        out_file
            .write_all(&output_vec)
            .expect("something went wrong writing the file.\ntell the dev: line 91 dectohex.rs");
    }
    Ok(())
}

pub fn convert_hex_file(
    path: String,
    out: bool,
    show_10: bool,
    mut new_path: String,
) -> Result<(), ()> {
    check_readability(&path);
    let mut contents: Vec<u8> = fs::read(path.clone()).unwrap();
    let mut string: String = String::new();
    eprintln!("\nHex values of {}:", path);

    if show_10 {
        // user defined option
        // removes eof and other carriage returns.
        contents.retain(|value| *value != 10);
    }
    //conversion to hex.
    for i in &contents {
        string.push_str(format!("{:#04x} ", i).as_str().strip_prefix("0x").unwrap());
    }

    println!("{}", string);

    eprintln!("\nInteger values: \n{:?}", contents);

    // handle file output if user defines.
    if out {
        new_path.push_str(".out");
        string.push_str("\n");
        match fs::exists(&new_path) {
            Ok(true) => (),
            Ok(false) => (),
            Err(e) => {
                eprintln!("An error occurred.\n{e}");
                exit(1);
            }
        }

        let mut out_file = File::create(new_path.clone()).unwrap();
        out_file.write_all(b"hex values: ").unwrap();
        out_file
            .write_all(string.clone().as_bytes())
            .expect("something went wrong writing the file.\ntell the dev: line 137 dectohex.rs");
        out_file.write_all(b"decimal values: ").unwrap();
        out_file
            .write_fmt(format_args!("{:?}\n", contents))
            .unwrap();
        out_file.write_all(b"plaintext: ").unwrap();
        out_file.write_all(&contents).expect(
            "something went wrong writing to the file,\ntell the dev: line 145 dectohex.rs",
        );
    }
    Ok(())
}
