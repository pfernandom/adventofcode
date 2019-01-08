use std::error::Error;
use common::read_file_lines;
use std::collections::HashMap;

pub fn run() ->  Result<(), Box<dyn Error>> {
    let lines = read_file_lines("./src/names.txt");

    let mut names = HashMap::new();

    let mut max = -9999;
    let mut curr_max = String::from("");

    for line in lines {
        let mut i = line.split(",");
        let ind = i.next().unwrap().to_string();
        let curr = ind.to_string();
        let counter = names.entry(ind).or_insert(1);
        *counter += 1;

        if counter > &mut max {
            max = *counter;
            curr_max = curr;
        }
    };

    for (k, v) in names.iter() {
        println!("--{}::{}", k, v);
    }
    println!("{} : {}", curr_max, max);
    Ok(())
}