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

fn old_policy(rec:&PasswordPolicy)->usize{
    let (min,max,key,pass) = rec;
    if pass.contains(*key){
        let mut loc_count = 0;
        for ch in pass.chars(){
            if *key == ch {
                loc_count +=1
            }
        }
        if loc_count >= *min && loc_count <= *max{
            1
        }
        else{
            0
        }
    }
    else{
        0
    }
}

fn new_policy(rec:&PasswordPolicy)->usize{
    let (pos1,pos2,key,pass) = rec;
    let mut loc_count = 0;
    if pass.contains(*key){
        for (i,ch) in pass.chars().enumerate(){
            if *key == ch {
                if (i+1) == *pos1{
                    loc_count += 1;
                }
                else if (i+1) == *pos2{
                    loc_count +=1;
                }
            }
        }
    }
    if loc_count == 1 {
        1
    }
    else{
        0
    }
}
fn main() {
    println!("Day two!");
    let record = read_input().unwrap();
    let mut count_old = 0;
    let mut count_new = 0;
    for policy in record{
        count_old += old_policy(&policy);
        count_new += new_policy(&policy);
    }
    println!("Total number of valid passwords according to old policy: {}",count_old);
    println!("Total number of valid passwords according to new policy: {}",count_new);
}
