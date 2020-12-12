use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_type<T>(_:T) -> String {
    return std::any::type_name::<T>().to_string();
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
//         .map(|line| line.and_then( 
//                 |v| Ok(
//                     v.to_string().parse::<i32>()
//                     )
//                 ).unwrap().unwrap()
//             )
        .map(|line| line.expect("Has text").parse::<i32>().expect("Contain i32"))
        .collect()
}

fn find_match(lines: Vec<i32>) -> i32 {
    let low = &lines[(lines.len() / 2)..];
    let high = &lines[.. (lines.len() /2)];
    let mut pair: [i32; 2] = [-1, -1];
    for (i, item) in low.iter().enumerate() {
        println!("Idx = {}: {}", i, item);
        for h_item in high.iter().rev() {
            if item + h_item == 2020 {
                pair[0] = *item;
                pair[1] = *h_item;
                println!("Found pair {} {}", &item, &h_item);
            }
        }
    }
    return pair[0] * pair[1];
}

fn find_tripplet(lines: Vec<i32>) -> i32 {
    let mut tripplet: [i32; 3] = [-1, -1, -1];

    for (i, item1) in lines.iter().enumerate() {
        let jlines = &lines[i + 1 ..];
       for (j, item2) in jlines.iter().enumerate() {
           let klines = &jlines[j + 1 ..];
           for(k, item3) in klines.iter().enumerate() {
               if item1 + item2 + item3 == 2020 {
                   return item1 * item2 * item3;
               }
           }
       }
    }

    return tripplet[0] * tripplet[1] * tripplet[2];
}

fn main() {
//     let x = get_type(String::from("12345").parse::<i32>().unwrap());
//     println!("{:?}", x);
    let mut lines = lines_from_file("day1.input");
    lines.sort_by(|a, b| b.cmp(a));
//     println!("Pair product {}", find_match(lines));
    println!("Pair product {}", find_tripplet(lines));
}
