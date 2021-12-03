use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const BIT_SIZE: usize = 12;
//const BIT_SIZE: usize = 5;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn binary_to_dec(bin_idx: &str) -> isize {
    return isize::from_str_radix(bin_idx, 2).unwrap();
}

fn main() {
    let filename = "advanced";
    //let filename = "simple";

    let lines = lines_from_file(filename).expect("Could not load lines");
    part1(lines.clone());
    part2(lines.clone());
}

fn total_up_list(lines: Vec<String>) -> Vec<(u32, u32)> {
    let mut list = vec![(0, 0); BIT_SIZE];
    for line in lines.iter() {
        let char_vec: Vec<char> = line.chars().collect();
        let mut i = 0;
        for chr in char_vec {
            let cu32 = chr.to_digit(10);
            let crnt = list[i];
            let new_tup: (u32, u32) = match cu32 {
                Some(0) => (crnt.0 + 1, crnt.1),
                _ => (crnt.0, crnt.1 + 1),
            };
            list[i] = new_tup;
            i += 1;
        }
    }
    list
}

fn determine_gamma(lines: Vec<String>) -> isize {
    let list = total_up_list(lines);

    let mut final_arr = [0; BIT_SIZE];
    let mut ctr = 0;
    for li in list {
        let left = li.0;
        let right = li.1;
        if left > right {
            final_arr[ctr] = 0;
        } else {
            final_arr[ctr] = 1;
        }
        ctr += 1;
    }
    let bits: String = final_arr.iter().map(ToString::to_string).collect();
    return binary_to_dec(&bits.to_owned());
}

fn determine_epsilon(lines: Vec<String>) -> isize {
    let list = total_up_list(lines);

    let mut final_arr = [0; BIT_SIZE];
    let mut ctr = 0;
    for li in list {
        let left = li.0;
        let right = li.1;
        if left < right {
            final_arr[ctr] = 0;
        } else {
            final_arr[ctr] = 1;
        }
        ctr += 1;
    }
    let bits: String = final_arr.iter().map(ToString::to_string).collect();
    return binary_to_dec(&bits.to_owned());
}

// --------------------------------------------------- part 2
fn oxy_winning_rules(left: Vec<String>, right: Vec<String>) -> Vec<String>{
    let winner = if left.len() > right.len() {
        left
    } else if left.len() < right.len() {
        right
    } else {
        right
    };
    winner
}

fn co2_winning_rules(left: Vec<String>, right: Vec<String>) -> Vec<String>{
    let winner = if left.len() > right.len() {
        right
    } else if left.len() < right.len() {
        left
    } else {
        left
    };
    winner
}

fn determine_part2_winner(lines: Vec<String>, position: i32, strategy: String) -> Vec<String> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines.iter() {
        let char_vec: Vec<char> = line.chars().collect();
        let mut i = 0;
        for chr in char_vec {
            if position != i {
                i += 1;
                continue;
            }
            let cu32 = chr.to_digit(10);
            match cu32 {
                Some(0) => left.push(line.clone()),
                _ => right.push(line.clone())
            };

            i += 1;
        }
    }

    let winner = match strategy.as_ref() {
        "oxy" => oxy_winning_rules(left, right),
        _ => co2_winning_rules(left,right)
    };

    winner
}

fn determine_win(lines: Vec<String>, strategy: String) -> isize {
    let mut breaker = 1;
    let mut position = 0;
    let mut remainder = lines.clone();
    while breaker < 10000 {
        remainder = determine_part2_winner(remainder.clone(), position, strategy.clone());
        if remainder.len() == 1 {
            println!("we done!");
            break;
        }
        position += 1;
        breaker += 1;
    }

    let bits: String = remainder.pop().unwrap();
    return binary_to_dec(&bits.to_owned());
}

// ---------------------------------------------------

fn part1(lines: Vec<String>) {
    let gamma = determine_gamma(lines.clone());
    let epsilon = determine_epsilon(lines.clone());
    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);
    println!("total: {:?}", gamma * epsilon);
}

/*
you want to go through each bit and given the most from that column of bits
steal those records and then move to the next bit. until only one bit remains
 */
fn part2(lines: Vec<String>) {
    println!("starting part 2 🚀");
    let oxy = determine_win(lines.clone(), String::from("oxy"));
    let co2 = determine_win(lines.clone(), String::from("co2"));
    println!("oxy: {:?}", oxy);
    println!("co2: {:?}", co2);
    println!("total: {:?}", oxy*co2);
}