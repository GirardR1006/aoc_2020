use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::time::Instant;

mod account{
    pub type T = Vec<usize>;
    pub fn new() -> T{
        Vec::new()
    }
}

fn read_input() -> Result<account::T,Error>{
    let mut acc = account::new();
    let f = File::open("input_day_one")?;
    let file = BufReader::new(f);
    for line in file.lines(){
        let a = line.unwrap().parse().unwrap();
        acc.push(a);
    }
    Ok(acc)
}

// naive method: double (or triple) for
fn find_2020_naive(acc: &account::T, is_three: bool) -> usize{
    let mut res = 0;
    for first_term in acc{
        for second_term in acc{
            if is_three{
                for third_term in acc{
                    if first_term + second_term + third_term == 2020 {
                        res = first_term*second_term*third_term;
                    }
                }
            }
            else{
                if first_term + second_term == 2020 {
                    res = first_term * second_term;
                }
            }
        }
    }
    res
}

//escape the loop when a result is found, since we are assured there is
//only one
fn find_2020_break(acc: &account::T, is_three: bool) -> usize{
    let mut res = 0;
    for first_term in acc{
        for second_term in acc{
            if is_three{
                for third_term in acc{
                    if first_term + second_term + third_term == 2020 {
                        res = first_term*second_term*third_term;
                        break;
                    }
                }
            }
            else{
                if first_term + second_term == 2020 {
                    res = first_term * second_term;
                    break;
                }
            }
        }
    }
    res
}

//do not compare already met combinations
fn find_2020_not_redundant(acc: &account::T, is_three: bool) -> usize{
    let mut res = 0;
    for (idx,first_term) in acc.iter().enumerate(){
        for (idx_prime,second_term) in acc[idx..].iter().enumerate(){
            if is_three{
                for third_term in &acc[idx_prime..]{
                    if first_term + second_term + third_term == 2020 {
                        res = first_term*second_term*third_term;
                        break;
                    }
                }
            }
            else{
                if first_term + second_term == 2020 {
                    res = first_term * second_term;
                    break;
                }
            }
        }
    }
    res
}


fn main() {
    let acc = read_input().unwrap();
    let now_naive = Instant::now();
    let res_naive = find_2020_naive(&acc, true);
    let elapsed_naive = now_naive.elapsed().as_millis();
    let now_break = Instant::now();
    let res_break = find_2020_break(&acc, true);
    let elapsed_break = now_break.elapsed().as_millis();
    let now_not_redundant = Instant::now();
    let res_not_redundant = find_2020_not_redundant(&acc, true);
    let elapsed_not_redundant = now_not_redundant.elapsed().as_millis();
    println!("Execution of naive approach took {}ms",elapsed_naive);
    println!("Execution of break approach took {}ms",elapsed_break);
    println!("Execution of not_redundant approach took {}ms",
             elapsed_not_redundant);
    if (res_naive == res_break) && (res_break == res_not_redundant){
        println!("All functions agree on their return value")
    }
}
