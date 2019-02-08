extern crate regex;
extern crate chrono;

pub mod common;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod names;

fn main() {
    // let _result = day3::part1and2();
    let result = day4::part1and2();
    // let result = names::run();
    match result {
        Ok(_) => {
            println!("Success.");
        },
        Err(err) => {
            println!("Error: {}", err);
        },
    }
}

