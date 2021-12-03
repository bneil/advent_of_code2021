use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

//const BIT_SIZE: usize = 12;
const BIT_SIZE: usize = 5;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn binary_to_dec(bin_idx: &str) -> isize {
    return isize::from_str_radix(bin_idx, 2).unwrap();
}

fn main() {
    //let filename = "advanced";
    let filename = "simple";
    let lines = lines_from_file(filename).expect("Could not load lines");
    //part1(lines);
    part2(lines);
}

fn total_up_list(lines: Vec<String>) -> Vec<(u32, u32)> {
    let mut list = vec![(0,0); BIT_SIZE];
    for line in lines.iter() {
        let char_vec: Vec<char> = line.chars().collect();
        let mut i = 0;
        for chr in char_vec {
            let cu32 = chr.to_digit(10);
            let crnt = list[i];
            let new_tup: (u32, u32) = match cu32 {
                Some(0) => (crnt.0+1, crnt.1),
                _ => (crnt.0, crnt.1+1),
            };
            list[i] = new_tup;
            i+=1;
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
        }else{
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
        }else{
            final_arr[ctr] = 1;
        }
        ctr += 1;
    }
    let bits: String = final_arr.iter().map(ToString::to_string).collect();
    return binary_to_dec(&bits.to_owned());
}

// --------------------------------------------------- part 2
fn determine_oxy(lines: Vec<String>) -> isize {
    let list = total_up_list(lines);

    let mut final_arr = [0; BIT_SIZE];
    let mut ctr = 0;
    for li in list {
        let left = li.0;
        let right = li.1;

        if left > right {
            final_arr[ctr] = 0;
        }else if left < right {
            final_arr[ctr] = 1;
        }else {
            final_arr[ctr] = 1;
        }

        ctr += 1;
    }
    let bits: String = final_arr.iter().map(ToString::to_string).collect();
    return binary_to_dec(&bits.to_owned());
}

// ---------------------------------------------------

fn part1(lines: Vec<String>) {
    let gamma = determine_gamma(lines.clone());
    let epsilon = determine_epsilon(lines.clone());
    println!("gamma: {:?}", gamma);
    println!("epsilon: {:?}", epsilon);
    println!("total: {:?}", gamma*epsilon);
}

/*
you want to go through each bit and given the most from that column of bits
steal those records and then move to the next bit. until only one bit remains
 */
fn part2(lines: Vec<String>) {
    let oxy = determine_oxy(lines.clone());
    //let co2 = determine_co2(lines.clone());
    println!("oxy: {:?}", oxy);
    //println!("co2: {:?}", co2);
    //println!("total: {:?}", oxy*co2);
}