use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.unwrap())
        .collect()
}

fn get_column_length(line: &String) -> usize {
    line.len()
}

fn product(results: Vec<usize>) -> usize {
    results.iter().product()
}

fn main() {
    let lines = lines_from_file("day03.input");
    let col_count = get_column_length(&lines[0]);
    println!("Column count = {}", col_count);

//     let trails_to_check = vec![(1, 1), (1, 2)];
    let trails_to_check = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut results = Vec::<usize>::new(); 
    let mut trail = Vec::<char>::new(); 

    let mut skip_count = 0;

    for t in trails_to_check {
        for (i, line) in lines.iter().enumerate() {
//             println!("Line {}", i);
            if i == 0 {
                continue;
            }

            if t.1 > 1 && (i % t.1) > 0 {
//                 println!("Skipped.");
                skip_count += 1;
                continue;
            }

            trail.push(line.chars().nth(t.0 * (i - skip_count) % col_count).unwrap());
//             println!("Read coord[{:?},{:?}]{:?}", i, t.0 * (i - skip_count) % col_count, trail);
        }
        results.push(trail.iter().filter(|v| **v == '#').count());
        println!("{:?}", results);
        trail.clear();
        skip_count = 0;
    }

    println!("Trees count = {:?}", product(results));

//     println!("Number of trees: {}", trail.iter().filter(|v| **v == '#').count());
}
