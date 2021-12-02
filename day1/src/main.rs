use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    return numbers
}

fn main() {
    let arr = [
     607,
     618,
     618,
     617,
     647,
     716,
     769,
     792
    ];

    let arr2 = load_from_file("src/input");

    let mut prev = -1;
    let mut total = 0;
    for i in arr2.iter() {
        if prev == -1 {
            prev = *i;
            continue;
        }
        if i > &prev {
            total += 1;
        }
        prev = *i
    }
    println!("increased: {:?}", total);

    // part 2
    let arr2size = arr2.len();
    let mut sliding = [0;2000];
    let mut i = 0;
    for num in arr2.iter() {
        if (i+1) >= arr2size || (i+2) >= arr2size {
            break
        }
        //println!("status: {:?}, {:?}, {:?}", i, arr2[i+1],arr2[i+2]);
        sliding[i] = num + arr2[i+1] + arr2[i+2];
        //println!("int: {:?}", i);
        i+=1;
    }

    prev = -1;
    total = 0;
    for i in sliding.iter() {
        if prev == -1 {
            prev = *i;
            continue;
        }
        if i > &prev {
            total += 1;
        }
        prev = *i
    }
    println!("increased2: {:?}", total);
}
