use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashMap,
};
use std::borrow::Borrow;

extern crate regex;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Cell {
    value: u32,
    hit: bool,
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let filename = "advanced";
    //let filename = "simple";

    let lines = lines_from_file(filename).expect("Could not load lines");
    //part1(lines.clone());
    part2(lines.clone());
}

fn part2(lines: Vec<String>) {
    let mut board_num = 0;
    let mut bingo_numbers: Vec<u32> = Vec::new();
    let mut boards: HashMap<i32, Vec<Vec<Cell>>> = HashMap::new();
    let mut board: Vec<Vec<Cell>> = vec![vec![Cell { value: 0, hit: false }; 5]; 5];

    board.clear();

    // the grand setup
    for line in lines.iter() {
        if line.contains(",") {
            bingo_numbers = read_bingo_line(&line, ",");
            continue;
        }
        if line.is_empty() {
            if board_num > 0 { // first
                boards.insert(board_num, board.clone());
                board.clear();
            }
            board_num += 1;
            continue;
        }
        let board_numbers = read_line(&line);
        //println!("{:?}", board_numbers);
        board.push(board_numbers)
    }
    boards.insert(board_num, board.clone());
    println!("loaded");

    //play bingo!
    let mut total = 0;
    let mut winners: HashMap<i32, i32> = HashMap::new();
    let max_winners = boards.len() - 1;

    'outer: for bingo_number in bingo_numbers.iter() {
        println!("calling number! {:?}", bingo_number);
        for (board_number, this_board) in boards.iter_mut() {
            if !previous_winner(board_number, &winners) {

                for (i, row) in this_board.iter_mut().enumerate() {
                    for (y, col) in row.iter_mut().enumerate() {
                        if &col.value == bingo_number {
                            col.hit = true
                        }
                    }
                }

                if winners.len() == max_winners {
                    print_board(this_board);
                    total = tally_win(this_board, bingo_number);
                    break 'outer;
                }

                if check_winner(this_board) {
                    winners.insert(*board_number, 1);
                    continue;
                }
            }
        }
    }

    for winner in winners.iter() {
        println!("winners: {:?}", winner)
    }

    //display win
    println!("total: {:?}", total)
}

fn previous_winner(board_number: &i32, prev_winners: &HashMap<i32, i32>) -> bool {
    prev_winners.contains_key(board_number)
}

fn part1(lines: Vec<String>) {
    let mut board_num = 0;
    let mut bingo_numbers: Vec<u32> = Vec::new();
    let mut boards: HashMap<i32, Vec<Vec<Cell>>> = HashMap::new();
    let mut board: Vec<Vec<Cell>> = vec![vec![Cell { value: 0, hit: false }; 5]; 5];

    board.clear();

    // the grand setup
    for line in lines.iter() {
        if line.contains(",") {
            bingo_numbers = read_bingo_line(&line, ",");
            continue;
        }
        if line.is_empty() {
            if board_num > 0 { // first
                boards.insert(board_num, board.clone());
                board.clear();
            }
            board_num += 1;
            continue;
        }
        let board_numbers = read_line(&line);
        //println!("{:?}", board_numbers);
        board.push(board_numbers)
    }
    boards.insert(board_num, board.clone());
    println!("loaded");

    for (board_number, this_board) in boards.iter_mut() {
        println!("board num: {}", board_number);
        print_board(this_board);
    }

    //play bingo!
    let mut total = 0;
    'outer: for bingo_number in bingo_numbers.iter() {
        println!("calling number! {:?}", bingo_number);
        for (board_number, this_board) in boards.iter_mut() {
            for (i, row) in this_board.iter_mut().enumerate() {
                for (y, col) in row.iter_mut().enumerate() {
                    if &col.value == bingo_number {
                        col.hit = true
                    }
                }
            }
            if bingo_number == &23 {
                print_board(this_board)
            }
            if check_winner(this_board) {
                total = tally_win(this_board, bingo_number);
                break 'outer;
            }
        }
    }

    //display win
    println!("total: {:?}", total)
}

fn print_board(this_board: &Vec<Vec<Cell>>) {
    for x in 0..5 {
        for y in 0..5 {
            let cell = this_board[x][y].clone();
            if cell.hit {
                print!("[{:?}] ", cell.value);
            } else {
                print!("{:?} ", cell.value);
            }
        }
        println!();
    }
    println!();
}

fn tally_win(this_board: &mut Vec<Vec<Cell>>, bingo_number: &u32) -> u32 {
    let mut ttl = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !this_board[x][y].hit {
                //println!("cell not hit {:?}", this_board[x][y]);
                ttl += this_board[x][y].value
            }
        }
    }
    println!("tallying winner: {:?} x {:?}", ttl, bingo_number);
    ttl * bingo_number
}

fn check_winner(this_board: &mut Vec<Vec<Cell>>) -> bool {
    let mut winning = false;
    let mut total = 0;

    //check row
    for x in 0..5 {
        let mut colm = Vec::new();
        for y in 0..5 {
            colm.push(this_board[x][y].hit)
        }

        winning = is_all_same(colm, true);
        if winning {
            println!("row win!");
            //print_board(this_board);
            break;
        }
    }

    //check col
    if !winning {
        for y in 0..5 {
            let mut colm = Vec::new();
            for x in 0..5 {
                colm.push(this_board[x][y].hit)
            }

            winning = is_all_same(colm, true);
            if winning {
                println!("col win!");
                break;
            }
        }
    }

    //check diag
    /*
    if !winning {
        let left_diag = vec![
            this_board[0][0].hit,
            this_board[1][1].hit,
            this_board[2][2].hit,
            this_board[3][3].hit,
            this_board[4][4].hit,
        ];
        let right_diag = vec![
            this_board[0][4].hit,
            this_board[1][2].hit,
            this_board[2][2].hit,
            this_board[3][1].hit,
            this_board[4][1].hit,
        ];
        winning = is_all_same(left_diag,true) || is_all_same(right_diag,true)
    }*/

    winning
}

fn is_all_same(arr: Vec<bool>, desired: bool) -> bool {
    let mut all_good = true;
    for a in arr {
        if a != desired {
            all_good = false
        }
    }
    all_good
}

fn read_bingo_line(l: &str, delim: &str) -> Vec<u32> {
    let split = l.split(delim);
    split.map(|x| x.parse::<u32>().unwrap()).collect()
}

fn read_line(l: &str) -> Vec<Cell> {
    let re = Regex::new(r"\s+").unwrap();
    let line = re.replace_all(l, " ");

    let raw_cell: Vec<u32> = line.trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    raw_cell.into_iter().map(|x: u32| Cell { value: x, hit: false }).collect()
}