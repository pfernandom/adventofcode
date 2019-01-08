use regex::Regex;
use std::error::Error;
use common::read_file_lines;
use std::collections::HashMap;

fn parse(my_str: &str) -> u32 {
    my_str.parse::<u32>().unwrap()
}

fn fill(state: &mut HashMap<String, u32>, from: (u32, u32), to:(u32, u32), id:u32){
    let x = from.0;
    let y = from.1;

    for i in x..to.0+1 {
        for j in y..to.1+1{
            //let ind = get_index(j, i, 1000) as usize;
            let ind = format!("{}-{}",j,i);
            match state.get(&ind) {
                Some(_) => {
                    &state.insert(ind, 0);
                },
                None => {
                    &state.insert(ind, id);
                },
            }
        }
    }
}

pub fn part1and2() ->  Result<(), Box<dyn Error>> {
    let lines = read_file_lines("./src/day3.txt");
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): +(\d+)x(\d+)").unwrap();
    let mut state: HashMap<String, u32> =  HashMap::new();
    let mut count_ids: HashMap<u32, u32> =  HashMap::new();

    for line in lines {
        for cap in re.captures_iter(&line) {
            let id = parse(&cap[1]);

            let left = parse(&cap[2]);
            let right = parse(&cap[3]);

            let width = parse(&cap[4]);
            let height = parse(&cap[5]);

            let from = (left, right);
            let to = (left+width-1, right+height-1);

            count_ids.insert(id, width*height);

            fill(&mut state, from, to, id);
        }
    }
    let mut count = 0;
    for el in state.values() {
        if *el == 0 {
            count = count + 1;
        }
        else {
            count_ids.entry(*el).and_modify(|e| { *e -= 1 });
        }
    }

    for (k, v) in count_ids {
        if v == 0 {
            println!("ID with no overlaps: {}", k);
        }
    }

    println!("Total of overlaps: {}", count);
    Ok(())
}