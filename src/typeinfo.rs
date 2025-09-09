use std::mem::size_of;
pub fn signed() {
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
pub fn unsigned(no_args: u8) {
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
pub fn floatingpoint(no_args: u8) {
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
pub fn convertbytes(
    vec: Vec<u8>,
    strip_prefix: bool,
    spaces: bool,
    add_machine_prefix: bool,
    rotate: i64,
) {
    let mut string: String = String::new();
    for i in vec {
        let mut output = String::new();
        match rotate {
            0 => output = crate::dectohex::convert_to_hex(i.into()),
            _ => output = crate::dectohex::rotate_char_to_hex(rotate, i.into()),
        }
        let mut before_return: String = String::new();

        if add_machine_prefix {
            before_return = format!("\\x{} ", output.strip_prefix("0x").unwrap());
        }
        if !add_machine_prefix {
            if strip_prefix {
                before_return = format!("{} ", output.strip_prefix("0x").unwrap());
            } else {
                before_return = format!("{} ", output);
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

/* perhaps create an implementation?*/
pub fn sizes() {
    println!("Primitive type sizes:");
    let unsigned = [
        ("u8", size_of::<u8>()),
        ("u16", size_of::<u16>()),
        ("u32", size_of::<u32>()),
        ("u64", size_of::<u64>()),
        ("u128", size_of::<u128>()),
    ];
    let signed = [
        ("i8", size_of::<i8>()),
        ("i16", size_of::<i16>()),
        ("i32", size_of::<i32>()),
        ("i64", size_of::<i64>()),
        ("i128", size_of::<i128>()),
    ];
    let floats = [("f32", size_of::<f32>()), ("f64", size_of::<f64>())];
    let strings = [
        ("String", size_of::<String>()),
        ("String slice", size_of::<&str>()),
    ];
    println!("\nUnsigned Integer Sizes:");
    unsigned
        .iter()
        .for_each(|(k, v)| println!("size of ({k}): {v} bytes"));
    println!("\nSigned Integer Sizes:");
    signed
        .iter()
        .for_each(|(k, v)| println!("size of ({k}): {v} bytes"));
    println!("\nFloating Point Sizes:");
    floats
        .iter()
        .for_each(|(k, v)| println!("size of ({k}): {v} bytes"));
    println!(
        "\nSize of Strings:\nsize of (Char): {} bytes\nsize of (String): (4*length)",
        size_of::<char>()
    );
    println!("\nString Sizes: [Rust specific]");
    strings
        .iter()
        .for_each(|(k, v)| println!("size of ({k}): {v} bytes"));
}
