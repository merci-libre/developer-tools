mod lib;
use lib::chmod_lib::StringFormat;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Permissions {
    pub readperms: bool,
    pub writeperms: bool,
    pub executeperms: bool,
}

pub fn chmod_perms_calculator(string: String) -> (String, String, BTreeMap<String, Permissions>) {
    // converts the string into a readable byte vector
    let mut string_vec: Vec<char> = string.to_lowercase().chars().collect();
    let mut perms_and_value: BTreeMap<String, Permissions> = BTreeMap::new();

    // values to be modified for hashmap
    let class = vec!["owner", "group", "public"];

    //iterator counters.
    let mut count = 0;
    let mut class_iter = class.into_iter();
    let mut value: u8 = 0;

    //empty variables for holding.
    let mut result = Vec::new();
    let mut class_title = String::new();
    let mut rperms = false;
    let mut wperms = false;
    let mut eperms = false;

    //format the string
    let mut string_iter = string_vec.format_chmod_value();

    //start iter
    for chars in &string_iter {
        let mut add = 0;

        match chars {
            'r' => {
                add = 4;
                rperms = true
            }
            'w' => {
                add = 2;
                wperms = true
            }
            'x' => {
                add = 1;
                eperms = true
            }
            _ => (),
        }

        count += 1;
        value += add;
        if count % 3 == 0 {
            class_title = match class_iter.next() {
                None => break,
                Some(x) => x.to_string(),
            };
            result.push(value);
            perms_and_value.insert(
                class_title,
                Permissions {
                    readperms: rperms,
                    writeperms: wperms,
                    executeperms: eperms,
                },
            );
            rperms = false;
            wperms = false;
            eperms = false;
            value = 0;
        }
    }
    //end iter

    let rs: String = result.iter().map(|&x| x.to_string()).collect();

    return (rs, string_iter.vec_to_string(), perms_and_value);
}
