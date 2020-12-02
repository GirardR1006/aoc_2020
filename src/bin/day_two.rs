use std::io::{BufRead,BufReader,Error};
use std::fs::File;

type PasswordPolicy = (usize,usize,char,String);

fn read_input() -> Result<Vec<PasswordPolicy>,Error>{
    let f = File::open("input_day_two")?;
    let mut record = Vec::new();
    let file = BufReader::new(f);
    for line in file.lines(){
        let line = line.unwrap();
        let mut splitted_line = line.split_whitespace();
        let pair = splitted_line.next().unwrap();
        let key = splitted_line.next().unwrap();
        let pass = splitted_line.next().unwrap();
        let min_max : Vec<&str> = pair.split('-').collect();
        let key_without_ : Vec<&str> =
            key.split(':').collect();
        record.push((
                min_max[0].parse().unwrap(),
                min_max[1].parse().unwrap(),
                key_without_[0].parse().unwrap(),
                String::from(pass)))
    }
    Ok(record)
}
fn main() {
    println!("Day two!");
    let record = read_input().unwrap();
    println!("Record: {:?}",record);
    let mut count = 0;
    for elt in record {
        let (min,max,key,pass) = elt;
        if pass.contains(key){
            let mut loc_count = 0;
            for ch in pass.chars(){
                if key == ch {
                    loc_count +=1
                }
            }
            if loc_count >= min && loc_count <= max{
                println!("min: {}",min);
                println!("max: {}",max);
                println!("key: {}",key);
                println!("pass: {}",pass);
                count += 1;
            }
        }
    }
    println!("Total number of valid passwords: {}",count);
}
