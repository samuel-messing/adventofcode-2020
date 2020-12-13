use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Map {
    grid: [[bool; 31]; 323],
}

impl Map {
    fn is_tree(&self, row: i32, column: i32) -> bool {
        return self.grid[row as usize][column as usize];
    }
}


fn main() {
    let mut raw_grid: [[bool; 31]; 323] = [[false; 31]; 323];
    if let Ok(lines) = read_lines("./input.txt") {
        let mut num_rows = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut num_cols = 0;
                for c in ip.chars() {
                    match c {
                        '#' => raw_grid[num_rows][num_cols] = true,
                        '.' => raw_grid[num_rows][num_cols] = false,
                        _ => continue,
                    }
                    num_cols += 1;
                }
            }
            num_rows += 1;
        }
    }

    let map = Map {
        grid: raw_grid,
    };

    let num_trees = count_trees(&map, 3, 1);

    println!("The answer to part one is: {}", num_trees);

    let mut all_answers = Vec::new();
    for (xslope, yslope) in [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)].iter() {
        all_answers.push(count_trees(&map, *xslope, *yslope));
    }

    println!("The answer to part two is: {}", all_answers.into_iter().fold(1, |a,b| a * b));
}

fn count_trees(map: &Map, xslope: i32, yslope: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    while y < 323 {
        if map.is_tree(y, x) {
            num_trees += 1;
        }
        if x + xslope > 30 {
            x = (x + xslope) - 31;
        } else {
            x += xslope;
        }
        y += yslope;
    }
    return num_trees;

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
