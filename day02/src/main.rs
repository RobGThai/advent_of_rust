use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

type Password = (i32, i32, String, String); // 1-3 a: abcde

fn extract_input(line: String) -> Password {
    let input: Vec<&str> = line.split(" ").collect();
    let range: Vec<&str> = input[0].split("-").collect();
    let pattern: String = input[1][.. input[1].find(":").expect("")].to_string();
    let p: Password = (range[0].parse::<i32>().expect("Contan minimum req"), 
                       range[1].parse::<i32>().expect("Contain maximum req"), 
                       pattern, 
                       input[2].to_string());
    return p;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Password> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| extract_input(line.expect("Has text")))
        .collect()
}

#[allow(dead_code)]
fn verify_password(p: &Password) -> bool {
    let c = p.3.matches(&p.2).count();
//     println!("Found {} in {}, {} times", p.2, p.3, c);
    return c >= p.0 as usize && c <= p.1 as usize
}

#[allow(dead_code)]
fn verify_tobaggan(p: &Password) -> bool {
    let mut count = 0;
    let ch1 = p.3.chars().nth((p.0 - 1) as usize).unwrap();
    let ch2 = p.3.chars().nth((p.1 - 1) as usize).unwrap();
    let code = p.2.chars().nth(0).unwrap();
    if ch1 == code {
        count += 1;
    }

    if ch2 == code {
        count += 1;
    }

    return count == 1;
}

fn main() {
    println!("Day02!");
    let lines = lines_from_file("day02.input");
    let c = lines.iter().filter(|l| verify_tobaggan(l)).count();
    
    println!("Number of valid passwords: {}", c);
}
