extern crate hex;

use std::{i64, str};

pub enum Type {
    CHAR,
    DEC,
    HEX,
}

pub fn add(values: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;

    for v in values {
        result += v;
    }
    return result
}

pub fn substract(values: &Vec<i64>) -> i64 {
    let mut result = values[0];

    for i in 1..values.len() {
        result -= values[i];
    }

    return result
}


pub fn dec_to_hex(val: i64) -> String {
    let hex = format!("{:#X}", val);
    return hex;
}

pub fn hex_to_dec(val: i64) -> String {
    return val.to_string();
}

pub fn hex_to_char(val: &str) -> Result<String, String> {
    let mut decoded = match hex::decode(val) {
        Ok(v) => v,
        Err(e) => return Err(format!("Problem converting hex to char\n{}", e))
    };

    decoded.reverse();
    let result = match str::from_utf8(&decoded) {
        Ok(v) => v,
        Err(e) => return Err(format!("Problem converting hex to char\n{}", e))
    };
    Ok(result.to_string())
}


pub fn convert(convert_to: Type, values: &mut Vec<String>) -> Result<String, String> {

    let mut values_int: Vec<i64> = Vec::new();
    let mut _add: bool = false;
    let mut _substract: bool = false;

    // Check if addition or substraction is desired
    for v in &*values {
        if v == "+" {
            _add = true;
            values.retain(|x| x != "+");
            break;
        }
        else if v == "-" {
            _substract = true;
            values.retain(|x| x != "-");
            break;
        }
    }

    // Check what type to convert into, make certain actions based on it
    let converted: String = match convert_to {
        Type::CHAR => {
            let mut result_string: String = String::new();

            let mut tmp: Vec<String> = values.iter()
                                        .map(|x| x.trim_start_matches("0x").to_string())
                                        .collect();
            for mut item in tmp {
                if item.len() % 2 != 0 {
                    item = format!("0{}", item);
                }

                let ch = match hex_to_char(&item) {
                    Ok(v) => v,
                    Err(e) => return Err(format!("{}", e))
                };
                result_string += format!("{}", ch).as_str();
            }

            result_string
        },
        Type::HEX => {
            let result: i64;

            // Vec<String> to Vec<i64> of hex values
            values_int = values.iter()
                                .map(|x| x.parse::<i64>().expect("Error while parsing"))
                                .collect();

            // Substract/add if desired, otherwise convert all numbers
            if _substract {
                result = substract(&values_int);
                dec_to_hex(result)
            }
            else if _add {
                result = add(&values_int);
                dec_to_hex(result)
            }
            else {
                let mut multiple_conversions: String = String::new();
                for value in values_int {
                    multiple_conversions += format!("{} ", dec_to_hex(value)).as_str();
                }
                multiple_conversions
            }
        },
        Type::DEC => {
            let result: i64;
            let tmp: Vec<&str> = values.iter()
                                        .map(|x| x.trim_start_matches("0x"))
                                        .collect();

            // Vec<String> to Vec<i64> of decimal values
            values_int = tmp.iter()
                            .map(|x| i64::from_str_radix(x, 16).expect("Error while parsing"))
                            .collect();

            // Substract/add if desired, otherwise convert all numbers
            if _substract {
                result = substract(&values_int);
                hex_to_dec(result)
            }
            else if _add {
                result = add(&values_int);
                hex_to_dec(result)
            }
            else {
                let mut multiple_conversions: String = String::new();
                for value in values_int {
                    multiple_conversions += format!("{} ", hex_to_dec(value)).as_str();
                }
                multiple_conversions
            }
        }
    };
    Ok(converted)
}
