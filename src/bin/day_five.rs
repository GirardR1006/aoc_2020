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
fn bin_div(row_seq: &str,lb: usize,ub:usize)
    -> usize{
    let mut lb = lb;
    let mut ub = ub;
    for code in row_seq.chars(){
        match code {
            'F'|'L' => ub = lb + (ub-lb)/2,
            'B'|'R' => lb = lb + (ub+1-lb)/2,
            _ => println!("POUET")
        };
    }
    ub
}

fn main() {
    println!("Day five!");
    let records = read_input().unwrap();
    let mut max_id = 0;
    let mut ids = Vec::new();
    for code in records.iter(){
        let row_id = bin_div(&code[..7],0,127);
        let col_id = bin_div(&code[7..10],0,7);
        let id = row_id*8 + col_id;
        ids.push(id);
        max_id = max(id,max_id);
    }
    ids.sort();
    let first_id = ids[0];
    let my_id = ids
        .iter()
        .fold(first_id, |acc, x|
              {match x - 1 == acc
                  {true => acc + 1,
                      false => acc}});
    println!("My id: {}",my_id+1);
}
