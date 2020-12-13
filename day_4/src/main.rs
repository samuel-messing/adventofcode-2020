use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i32>,
}

impl Passport {
    fn from_map(values: HashMap<String, String>) -> Passport {
        return Passport {
            byr: match values.get("byr") {
                None => None,
                Some(v) => v.parse::<i32>().ok(),
            },
            iyr: match values.get("iyr") {
                None => None,
                Some(v) => v.parse::<i32>().ok(),
            },
            eyr: match values.get("eyr") {
                None => None,
                Some(v) => v.parse::<i32>().ok(),
            },
            hgt: match values.get("hgt") {
                None => None,
                Some(v) => Some(v.clone()),
            },
            hcl: match values.get("hcl") {
                None => None,
                Some(v) => Some(v.clone()),
            },
            ecl: match values.get("ecl") {
                None => None,
                Some(v) => Some(v.clone()),
            },
            pid: match values.get("pid") {
                None => None,
                Some(v) => Some(v.clone()),
            },
            cid: match values.get("cid") {
                None => None,
                Some(v) => v.parse::<i32>().ok(),
            },
        };
    }
    fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }
}

fn main() {
    let mut passports: Vec<Passport> = Vec::new();
    let key_val_re = Regex::new(r"([a-z0-9]*:#?[a-z0-9]*)").unwrap();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut values: HashMap<String, String> = HashMap::new();
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    let passport = Passport::from_map(values);
                    passports.push(passport);
                    values = HashMap::new();
                    continue;
                }
                for cap in key_val_re.captures_iter(&ip) {
                    for i_cap in cap.iter() {
                        let k_v: Vec<String> = i_cap
                            .unwrap()
                            .as_str()
                            .split(":")
                            .map(|a| a.to_string())
                            .collect();
                        values.insert(k_v[0].clone(), k_v[1].clone());
                    }
                }
            }
        }
        // there's one more...
        let passport = Passport::from_map(values);
        passports.push(passport);
    }

    let mut num_valid = 0;
    for passport in passports {
        if passport.is_valid() {
            num_valid += 1;
        }
    }
    println!("The answer to part one is: {}", num_valid);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
