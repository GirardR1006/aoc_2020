#![feature(map_into_keys_values)]

use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::collections::HashMap;

fn compute_frm_answers(seq : Vec<char>,
                       num_occur :usize) ->
HashMap<char,usize>{
    let letters = 'a'..='z';
    let mut frm = HashMap::new();
    for letter in letters{
        if seq.contains(&letter){
            let loc_num_occur : usize = seq
                .iter()
                .filter(|&x| *x == letter)
                .count();
            if num_occur == loc_num_occur{
                match frm.insert(letter,1) {
                    Some(_) =>
                        println!("Duplicated key"),
                    None => ()
                }
            };
        }
        else{
            match frm.insert(letter,0) {
                Some(_) =>
                    println!("Duplicated key"),
                None => ()
            };
        }
    }
    frm
}

fn read_input() ->
Result<Vec<HashMap<char,usize>>,Error>{
    let f = File::open("input_day_six")?;
    let mut forms = Vec::new();
    let file = BufReader::new(f);
    let mut group = String::from("");
    let mut count = 0;
    for line in file.lines(){
        let to_process = line.unwrap();
        if to_process == String::from(""){
            let charseq = group.chars().collect();
            forms
                .push(compute_frm_answers(charseq, count));
            group.clear();
            count = 0;
        }
        else{
            group.push_str(" ");
            count += 1;
            group.push_str(to_process.as_str());
        }
    }
    Ok(forms)
}

fn main(){
    let forms = read_input();
    println!("sum of all answered questions: {}",
             forms
             .unwrap()
             .iter()
             .fold(0,|acc, x|{
                 let sum : usize = x
                     .values()
                     .sum();
                 acc + sum}));
}
