use regex::Regex;
use std::error::Error;
use common::read_file_lines;
use std::collections::HashMap;

pub fn part1and2() ->  Result<(), Box<dyn Error>> {
    let mut lines = read_file_lines("./src/day4.txt");
    lines.sort_unstable();
    let re_start_shift = Regex::new(r"(\d{2}):(\d{2})] Guard #(\d+) begins shift").unwrap();
    let re_asleep = Regex::new(r"(\d{2}):(\d{2})] falls asleep").unwrap();
    let re_wake = Regex::new(r"(\d{2}):(\d{2})] wakes up").unwrap();

    let mut guards : HashMap<String, Vec<(i16, i16)>> = HashMap::new();

    let mut from = 0;
    let mut current_guard = "".to_string();

    for line in lines {
        if re_start_shift.is_match(&line) {
            for cap in re_start_shift.captures_iter(&line) {
                // println!("hour: {}. minute:{}, id:{}", &cap[1], &cap[2], &cap[3]);
                current_guard = cap[3].to_string();
            }
        }

        if re_asleep.is_match(&line) {
            for cap in re_asleep.captures_iter(&line) {
                println!("hour: {}. minute:{}, sleep :)", &cap[1], &cap[2]);
                from = if &cap[1] == "00" { cap[2].parse::<i16>().unwrap() } else { 0 - cap[3].parse::<i16>().unwrap() }
            }
        }

        if re_wake.is_match(&line) {
            for cap in re_wake.captures_iter(&line) {
                // println!("hour: {}. minute:{}, awake (slept from to {:?})", &cap[1], &cap[2], (from, &cap[2]));
                let time = (from, cap[2].parse::<i16>().unwrap());
                if guards.contains_key(&current_guard) {
                    let times_for_guard = guards.get_mut(&current_guard).unwrap();
                    times_for_guard.push(time);
                }
                else {
                    guards.insert(current_guard.clone(), vec!(time));
                }
            }
        }

    }

     let mut minutes: [i16; 60] = [0; 60];
     let mut max_minutes = 0;
     let mut top_minute = 0;
     let mut max_guard = String::from("");

     let mut max_count_per_mint = 0;
     let mut current_top_mcpm = 0;
     let mut guard_for_mcpm = String::from("");

    println!("{:?}", guards);
    for (guard, times) in &guards {
        println!("For guard {}, these are the times: {:?}", guard, times);
        let mut current_top = 0;
        let mut current_max_count = 0;
        let mut current_minutes = 0;
        for (start, end) in times {
            for minute in *start..*end {
                let m = minute as usize;
                // add to current minute 'm'
                minutes[m] = minutes[m] + 1;
                // add to the total of minutes for this guard
                current_minutes = current_minutes + 1;
                // if current max count per minute is less than current minute
                if current_max_count < minutes[m] {
                    current_top = minute;
                    current_max_count = minutes[m];
                }
                
                // to calculate the second part
                if max_count_per_mint < current_max_count {
                    max_count_per_mint = current_max_count;
                    current_top_mcpm = current_top;
                    guard_for_mcpm = guard.clone();
                }
                println!("For {} minute, now we have {}. current_top={} current_max_count={}", minute, minutes[m], current_top, current_max_count);
            }
        }

        if current_minutes > max_minutes {
            println!("{} minutes is larger that the current max {}", current_minutes, max_minutes);
            top_minute = current_top;
            max_minutes = current_minutes;
            max_guard = guard.clone();
            println!("top_minute={}, max_guard={}", top_minute, max_guard);
        }

        println!("For guard {}, there where a total of {} minutes", guard, current_minutes);
        minutes = [0; 60];
    }

    println!("Top minute: {}, Guard: {}, multiplied they're {:?}", top_minute, max_guard, top_minute as i32*max_guard.parse::<i32>().unwrap());

    println!("Part 2: current_top_mcpm={} guard_for_mcpm={}, max_count_per_mint={}, multiplied={}", current_top_mcpm, guard_for_mcpm,  max_count_per_mint, current_top_mcpm as i32*guard_for_mcpm.parse::<i32>().unwrap());

    Ok(())
}