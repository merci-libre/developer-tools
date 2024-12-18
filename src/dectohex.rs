use std::num::ParseIntError;
pub fn convert_to_hex(number: u128) -> String {
    let result = format!("{:#x}", number);
    return result;
}
pub fn convert_to_dec_from_hex(hex: String) -> Result<(), ParseIntError> {
    let decimal = u128::from_str_radix(hex.as_str(), 16)?;
    println!("{}", decimal);
    Ok(())
}
