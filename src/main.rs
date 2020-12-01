use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::vec::Vec;


fn read_input() -> Result<Vec<usize>,Error>{
    let mut account = Vec::new();
    let f = File::open("input_day_one")?;
    let file = BufReader::new(f);
    for line in file.lines(){
        let a = line.unwrap().parse().unwrap();
        account.push(a);
    }
    Ok(account)
}

// naive method: double for
fn find_two_2020(account: &Vec<usize>) -> usize{
    let mut res = 0;
    for first_term in account{
        for second_term in account{
            if first_term + second_term == 2020 {
                res = first_term * second_term;
            }
        }
    }
    res

}

fn main() {
    let account = read_input().unwrap();
    println!("Result of the multiplication of two numbers: {}",
             find_two_2020(&account));
}
