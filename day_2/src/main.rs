use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct OldPasswordPolicy {
    letter: char,
    min_times: i32,
    max_times: i32,
}

impl OldPasswordPolicy {
    fn is_valid(&self, password: &String) -> bool {
        let num_times = (password.len() - password.replace(self.letter, "").len()) as i32;
        return self.max_times >= num_times && num_times >= self.min_times;
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    index_a: usize,
    index_b: usize,
}

impl PasswordPolicy {
    fn is_valid(&self, password: &String) -> bool {
        if self.index_a >= password.len() || self.index_b >= password.len() {
            return false;
        }
        let first_char = password.as_bytes()[self.index_a] as char;
        let second_char = password.as_bytes()[self.index_b] as char;
        return (self.letter == first_char) ^ (self.letter == second_char);

    }
}


fn main() {
    let mut passwords_and_policies = Vec::new();
    let re = Regex::new(r"^(\d*)-(\d*) ([a-z]): ([a-z]*)$").unwrap();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for cap in re.captures_iter(&ip) {
                    let old_policy = OldPasswordPolicy {
                        letter: cap[3].parse::<char>().unwrap(),
                        min_times: cap[1].parse::<i32>().unwrap(),
                        max_times: cap[2].parse::<i32>().unwrap(),
                    };
                    let new_policy = PasswordPolicy {
                        letter: cap[3].parse::<char>().unwrap(),
                        index_a: cap[1].parse::<usize>().unwrap() - 1,
                        index_b: cap[2].parse::<usize>().unwrap() - 1,
                    };
                    passwords_and_policies.push(
                        (old_policy, new_policy, cap[4].to_string()));
                }
            }
        }
    }

    let mut num_valid = 0;
    for (policy, _, password) in &passwords_and_policies {
        if policy.is_valid(password) {
            num_valid += 1;
        }
    }
    println!("The answer to part one is: {}", num_valid);

    let mut num_new_valid = 0;
    for (_, policy, password) in &passwords_and_policies {
        if policy.is_valid(password) {
            num_new_valid += 1;
        }
    }
    println!("The answer to part one is: {}", num_new_valid);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
