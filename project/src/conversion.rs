use std::{i64, str};

pub enum Type {
    CHAR,
    DEC,
    HEX,
}

pub fn add(values: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for v in values {
        result += v;
    }
    return result
}

pub fn substract(values: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    for v in values {
        result -= v;
    }
    return result
}


pub fn dec_to_hex(val: i32) -> String {
    let hex = format!("{:#X}", val);
    return hex;
}

pub fn hex_to_dec(val: i32) -> String {
    return val.to_string();
}


pub fn convert(convert_to: Type, values: &mut Vec<String>) -> Result<String, &'static str> {

    let mut values_int: Vec<i32> = Vec::new();
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
        Type::CHAR => todo!(),
        Type::HEX => {
            let result: i32;

            // Vec<String> to Vec<i32> of hex values
            values_int = values.iter()
                                .map(|x| x.parse::<i32>().unwrap())
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
            let result: i32;
            let tmp: Vec<&str> = values.iter()
                                        .map(|x| x.trim_start_matches("0x"))
                                        .collect();

            // Vec<String> to Vec<i32> of decimal values
            values_int = tmp.iter()
                            .map(|x| i32::from_str_radix(x, 16).unwrap())
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

