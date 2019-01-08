use regex::Regex;
use std::error::Error;
use common::read_file_lines;
use chrono::{NaiveDateTime};

enum State {
    AWAKE,
    ASLEEP,
}

pub fn part1and2() ->  Result<(), Box<dyn Error>> {
    let mut lines = read_file_lines("./src/day4.txt");
    lines.sort_unstable();
    let re_start_shift = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut current_state = State::AWAKE;
    let mut curr_id = "";

    for line in lines {
        println!("{}", line);
        if re_start_shift.is_match(&line) {
            current_state = State::AWAKE;
            for cap in re_start_shift.captures_iter(&line) {
                curr_id = &cap[1]
            }
        }

    }

    Ok(())
}