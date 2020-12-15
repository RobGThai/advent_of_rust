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

// fn get_char_at(pos: i32, from: &String) -> char {
// }

fn main() {
    let lines = lines_from_file("test.input");
    let col_count = get_column_length(&lines[0]);
    println!("Column count = {}", col_count);

    let r:usize = 3;
    let mut trail = Vec::<char>::new(); 

    for (i, line) in lines.iter().enumerate() {
       if i == 0 {
           continue;
       }

       trail.push(line.chars().nth(r * i % col_count).unwrap());
    }

    println!("{:?}", trail);
    println!("Number of trees: {}", trail.iter().filter(|v| **v == '#').count());
}
