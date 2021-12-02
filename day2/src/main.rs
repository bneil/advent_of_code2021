use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("advanced").expect("Could not load lines");
    //part1(lines);
    part2(lines);
}

fn part2(lines: Vec<String>) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for line in lines {
        let vec: Vec<&str> = line.split(' ').collect();
        let action = vec[0];
        let movement = vec[1].parse::<i64>().unwrap();
        match action {
            "down" => {
                //depth += movement;
                aim += movement;
                println!("down {:?}: d: {:?}, a: {:?}", movement, depth, aim);
            },
            "up" => {
                //depth -= movement;
                aim -= movement;
                println!("up {:?}: d: {:?}, a: {:?}", movement, depth, aim);
            },
            "forward" => {
                horizontal += movement;
                depth += aim * movement;
                println!("forward {:?}: a: {:?}, h: {:?}, d: {:?}", movement, aim, horizontal, depth);
            },
            _ => {}
        }
        //println!("{:?}, {:?}, {:?}", aim, horizontal, depth);
    }
    let total = horizontal * depth;
    println!("horizontal: {:?}, depth: {:?} final: {:?}", horizontal, depth, total);
}
fn part1(lines: Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in lines {
        let vec: Vec<&str> = line.split(' ').collect();
        let action = vec[0];
        let movement = vec[1].parse::<i64>().unwrap();
        match action {
            "forward" => horizontal += movement,
            "down" => depth += movement,
            "up" => depth -= movement,
            _ => {}
        }
        println!("{:?}, {:?}", action, movement);
    }
    let total = horizontal * depth;
    println!("horizontal: {:?}, depth: {:?} final: {:?}", horizontal, depth, total);
}
