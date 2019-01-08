use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

pub fn day2() -> Result<()> {
    let filename = "./src/day2.txt";
    let file = File::open(filename)?;

    let lines: Vec<String> = BufReader::new(&file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let mut count2 = 0;
    let mut count3 = 0;
    for line in lines {
        let char_list: Vec<char> = line.chars().collect();
        println!(">{}", line);
        let mut cache: HashMap<char, u16> = HashMap::new();
        for c in char_list {
            let count = match cache.get(&c) {
                Some(v) => *v,
                None => 0 as u16,
            };
            cache.insert(c, count+1);
        }
        let mut has2 = false;
        let mut has3 = false;
        for (_, value) in cache {
            if value == 2 && !has2 {
                count2 = count2 + 1;
                has2 = true;
            }
            if value == 3 && !has3 {
                count3 = count3 + 1;
                has3 = true;
            }
        }
    }

    print!("Result = {}", count2 * count3);

    Ok(())
}

fn splice(str :&String, index :usize) -> String {
    [&str[..index], &str[index+1..]].concat()
}
pub fn day2_part2() -> Result<()> {
    let filename = "./src/day2.txt";
    let file = File::open(filename)?;

    let lines: Vec<String> = BufReader::new(&file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let mut cache: HashMap<String, String> = HashMap::new();
    for line in lines {
        let mut count = 0;
        for _ in line.chars() {
            let spliced = splice(&line, count);
            if cache.contains_key(&spliced) {
                let cached_line = cache.get(&spliced).unwrap();
                if cached_line != &line {
                    println!("RESULT = {} :: \n\t{} and \n\t{}",&spliced, &cached_line, &line);
                    return Ok(())
                }
            }
            else {
                cache.insert( spliced, line.clone());
            }
            count = count + 1;
        }
    }


    Ok(())
}

