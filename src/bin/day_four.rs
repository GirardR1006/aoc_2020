use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn check_nums(n:&String,lb:usize,ub:usize) -> bool{
    let v = (*n).parse::<usize>().unwrap();
    v < ub && v > lb
}
fn check_height(n:&String) -> bool{
    if n.len() == 4{
        let str_n = n.as_str();
        str_n.chars().nth(3).unwrap() == 'n' &&
            str_n.chars().nth(2).unwrap() == 'i' &&
            str_n[..2].parse::<usize>().unwrap() > 59 &&
            str_n[..2].parse::<usize>().unwrap() < 76
    }
    else if n.len() == 5{
        let str_n = n.as_str();
        str_n.chars().nth(4).unwrap() == 'm' &&
            str_n.chars().nth(3).unwrap() == 'c' &&
            str_n[..3].parse::<usize>().unwrap() > 150 &&
            str_n[..3].parse::<usize>().unwrap() < 193
    }
    else {false}
}

fn check_hcl(n:&String) -> bool{
    if (*n).len() == 7{
        let re = Regex::new(r"[0-9]+[a-f]$").unwrap();
        n.chars().nth(0).unwrap() == '#' &&
            re.is_match(&(n.as_str()[1..]))
    }
    else{false}
}

fn check_ecl(n:&String) -> bool{
    let re = Regex::new(r"(\bamb\b|\bblu\b|\bbrn\b|\bgry\b|\bgrn\b|\bhzl\b|\both\b)").unwrap();
    re.is_match(&(n.as_str()))
}

fn check_pid(n:&String) -> bool{
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(&(n.as_str()))
}

fn check_passport(passfields:&HashMap<String,String>)
    -> bool{
        let ch_byr = match passfields.get("byr"){
            Some(v) => check_nums(v, 1920, 2002),
            None => false
        };
        let ch_iyr = match passfields.get("iyr"){
            Some(v) => check_nums(v, 2010, 2020),
            None => false
        };
        let ch_eyr = match passfields.get("eyr"){
            Some(v) => check_nums(v, 2020, 2030),
            None => false
        };
        let ch_hgt = match passfields.get("hgt"){
            Some(v) => check_height(v),
            None => false
        };
        let ch_hcl = match passfields.get("hcl"){
            Some(v) => check_hcl(v),
            None => false
        };
        let ch_ecl = match passfields.get("ecl"){
            Some(v) => check_ecl(v),
            None => false
        };
        let ch_pid = match passfields.get("pid"){
            Some(v) => check_pid(v),
            None => false
        };

        ch_hcl && ch_hgt && ch_eyr &&  ch_iyr && ch_byr &&
            ch_ecl && ch_pid
}

fn read_input() ->
Result<Vec<HashMap<String,String>>,Error>{
    let f = File::open("input_day_four")?;
    let mut records = Vec::new();
    let file = BufReader::new(f);
    let mut potential_passport = String::from("");
    for line in file.lines(){
        let to_process = line.unwrap();
        if to_process == String::from(""){
            // potential passport is now complete,
            // perform verification
            let mut record = HashMap::new();
            let pass_str = &potential_passport;
            let sliced = pass_str.split_ascii_whitespace();
            for kv in sliced{
                let k = String::from(kv
                                     .split(':')
                                     .next()
                                     .unwrap());
                let v = String::from(kv
                                     .split(':')
                                     .next()
                                     .unwrap());
                &mut record.insert(k, v);
            };
            records.push(record);
            potential_passport.clear();
        }
        else{
            potential_passport
                .push_str(" ");
            potential_passport
                .push_str(to_process.as_str());
        }
    }
    Ok(records)
}

fn main() {
    println!("Day four!");
    let records = read_input().unwrap();
    println!("Total number of valid passports: {}",
             records
             .iter()
             .fold(0,|acc, m| {
                 if check_passport(m){acc + 1}
                 else {acc}} ));
}
