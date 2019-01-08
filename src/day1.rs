use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

pub fn day1() -> Result<()> {
    let re = Regex::new(r"(\+|\-)(\d+)").unwrap();
    let filename = "./src/day1.txt";
    let file = File::open(filename)?;
    /*
    for line in BufReader::new(file).lines() {
        //println!("{}", &line?);
        for cap in re.captures_iter(&line?) {
            println!("Sign: {} Number: {}", &cap[1], &cap[2]);
        }
    }*/
    let mut frecuencies = HashMap::new();

    let mut result: i32 = 0;
    let mut first_repeated_frec: i32 = 0;
    let mut has_repeated_frec: bool = false;

    let mut lines = Vec::new();
    for line in BufReader::new(&file).lines() {
        lines.push(line?);
    }

    while !has_repeated_frec {
        for line in lines.iter() {
            for cap in re.captures_iter(&line) {
                // println!("Sign: {} Number: {}", &cap[1], &cap[2]);
                // println!("Is add? {}", &cap[1] == "+");
                let this_num = &cap[2].parse::<i32>().unwrap();
                if &cap[1] == "+" {
                    result = result + this_num;
                } else {
                    result = result - this_num;
                }

                let key = result.to_string();
                println!("R:{}", key);
                if frecuencies.contains_key(&key) {
                    // println!("Contains KEY {}!", key);
                    first_repeated_frec = result;
                    has_repeated_frec = true;
                    break;
                } else {
                    // println!("{} DOESMT Contains KEY!", key);
                    frecuencies.insert(key, true);
                }
            }
            if has_repeated_frec {
                break;
            }
        }
    }

    println!("Result:{}", first_repeated_frec);
    Ok(())
}