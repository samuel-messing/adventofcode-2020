use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers.insert(ip.parse::<i32>().unwrap());
            }
        }
    }

    // Part 1
    for n1 in &numbers {
        for n2 in &numbers {
            if n1 + n2 == 2020 {
                println!("The answer to part one is: {} (numbers: {} and {})", n1 * n2, n1, n2);
            }
        }
    }

    // Part 2
    for n1 in &numbers {
        for n2 in &numbers {
            for n3 in &numbers {
                if n1 + n2 + n3 == 2020 {
                    println!("The answer to part two is: {} (numbers: {}, {}, and {})", n1 * n2 * n3, n1, n2, n3);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
