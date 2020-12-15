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

fn main() {
    let lines = lines_from_file("test.input");
    for line in lines {
        println!("{}", line);
    }
    println!("Hello, world!");
}
