use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

fn check_nums(n:&String,lb:usize,ub:usize) -> bool{
    let v = (*n).parse::<usize>().unwrap();
    v <= ub && v >= lb
}
fn check_hgt(value: &String) -> bool {
    let re = Regex::new(r"cm|in$").unwrap();
    let unit = match re.captures(&value) {
        Some(txt) => txt,
        None => {return false;},
    };
    let unit = &unit[0];
    let re = Regex::new(r"^\d+").unwrap();
    let value = re.captures(&value).unwrap();
    let value:i32 = value[0].parse().unwrap();
    if unit == "cm" {
        value >= 150 && value <= 193
    }
    else {
        value >= 59 && value <= 76
    }
}
fn check_hcl(n:&String) -> bool{
    let re = Regex::new(r"^#[0-9a-f]{6}$")
        .unwrap();
    re.is_match(&n)
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
        println!("ch_byr: {}",ch_byr);
        let ch_iyr = match passfields.get("iyr"){
            Some(v) => check_nums(v, 2010, 2020),
            None => false
        };
        println!("ch_iyr: {}",ch_iyr);
        let ch_eyr = match passfields.get("eyr"){
            Some(v) => check_nums(v, 2020, 2030),
            None => false
        };
        println!("ch_eyr: {}",ch_eyr);
        let ch_hgt = match passfields.get("hgt"){
            Some(v) => check_hgt(v),
            None => false
        };
        println!("ch_hgt: {}",ch_hgt);
        let ch_hcl = match passfields.get("hcl"){
            Some(v) => check_hcl(v),
            None => false
        };
        println!("ch_hcl: {}",ch_hcl);
        let ch_ecl = match passfields.get("ecl"){
            Some(v) => check_ecl(v),
            None => false
        };
        println!("ch_ecl: {}",ch_ecl);
        let ch_pid = match passfields.get("pid"){
            Some(v) => check_pid(v),
            None => false
        };
        println!("ch_pid: {}",ch_pid);
        println!("###########");

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
                let mut c_kv = kv.split(':');
                let k = String::from(c_kv
                                     .next()
                                     .unwrap());
                let v = String::from(c_kv
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
