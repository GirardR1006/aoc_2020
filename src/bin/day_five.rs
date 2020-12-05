use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::cmp::max;


fn read_input() ->
Result<Vec<String>,Error>{
    let f = File::open("input_day_five")?;
    let mut records = Vec::new();
    let file = BufReader::new(f);
    for line in file.lines(){
        let code = line.unwrap();
        if code.chars().count() != 10{
            println!("Error, input malformed");
            std::process::exit(-1);
        }
        else {
            records.push(code);
        }
    }
    Ok(records)
}
//find which row corresponds
//to the code
fn bin_div_row(row_seq: &str) -> usize{
    let mut lb = 0;
    let mut ub = 127;
    println!("Row_seq: {}", row_seq);
    for code in row_seq.chars(){
        match code {
            'F' => ub = lb + (ub-lb)/2,
            'B' => lb = lb + (ub+1-lb)/2,
            _ => println!("POUET")
        };
    }
    println!("ub: {}, lb: {}", ub, lb);
    ub
}
fn bin_div_col(col_seq: &str) -> usize{
    let mut lb = 0;
    let mut ub = 7;
    println!("col_seq: {}", col_seq);
    for code in col_seq.chars(){
        match code {
            'L' => ub = lb + (ub-lb)/2,
            'R' => lb = lb + (ub+1-lb)/2,
            _ => println!("POUET")
        };
    }
    println!("ub: {}; lb: {}",ub, lb);
    ub
}

fn main() {
    println!("Day five!");
    let records = read_input().unwrap();
    let mut max_id = 0;
    for code in records.iter(){
        let row_id = bin_div_row(&code[..7]);
        let col_id = bin_div_col(&code[7..10]);
        max_id = max(row_id*8 + col_id,max_id)
    }
    println!("Maximum id in record: {}",max_id);
}
