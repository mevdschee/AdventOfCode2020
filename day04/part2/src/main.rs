use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut i = 0;
    for passport in content.split("\n\n") {
        let mut properties = HashMap::new();
        for property in passport.split_whitespace() {
            let mut kv = property.splitn(2, ":");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            if key == "cid" {
                continue;
            }
            let restr = match key {
                "hgt" => "^[0-9]+(cm|in)$",
                "hcl" => "^#[0-9a-f]{6}$",
                "ecl" => "^(amb|blu|brn|gry|grn|hzl|oth)$",
                "pid" => "^[0-9]{9}$",
                _ => "^[0-9]{4}$",
            };
            let re = Regex::new(restr).unwrap();
            if !re.is_match(value) {
                break;
            }
            let valchr: String = value.chars().filter(|a| a.is_alphabetic()).collect();
            let numchr: String = value.chars().filter(|a| a.is_ascii_digit()).collect();
            let num: i32 = numchr.parse().unwrap_or(0);
            let valid = match (key, &*valchr) {
                ("byr", "") => num >= 1920 && num <= 2002,
                ("iyr", "") => num >= 2010 && num <= 2020,
                ("eyr", "") => num >= 2020 && num <= 2030,
                ("hgt", "cm") => num >= 150 && num <= 193,
                ("hgt", "in") => num >= 59 && num <= 76,
                _ => true,
            };
            if !valid {
                break;
            }
            if properties.contains_key(key) {
                properties.get(key).replace(&value);
            } else {
                properties.insert(key, value);
            }
        }
        if properties.len() == 7 {
            //println!("{:?}", properties);
            i += 1;
        }
    }
    println!("{}", i);
}
