extern crate nom;

use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use nom::IResult;
use nom::bytes::complete::take_while1;

#[derive(Debug)]
enum Instruction{
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}
#[derive(Debug)]
struct Program{
    instructions: Vec<(Instruction,bool)>,
    current_instruction: i32,
    count: i32
}
impl Program{
    fn execute(self: &mut Program) -> i32{
        let mut infinite = false;
        while !infinite {
            let (cur_instr,switch) = &mut self.instructions[
                self.current_instruction as usize];
            if *switch {infinite = true}
            else{
                match cur_instr {
                    Instruction::Acc(i) =>{
                        *switch = true;
                        self.count += *i;
                        self.current_instruction += 1;
                    },
                    Instruction::Jmp(i) =>{
                        *switch = true;
                        self.current_instruction += *i;
                    },
                    Instruction::Nop(_i) =>{
                        *switch = true;
                        self.current_instruction += 1;
                    }
                }
            };
        }
        println!("Infinite loop spotted.");
        self.count
    }
}

//Parser for one sequence
fn no_space(s: &str)
    -> IResult<&str,&str>{
        take_while1(|c| c !=' ')(s)
}
fn space(s: &str)
    -> IResult<&str,&str>{
        take_while1(|c| c ==' ')(s)
}
fn sign(s: &str)
    -> IResult<&str,&str>{
        take_while1(|c: char| c=='+' || c=='-')(s)
    }
fn instruction(input: &str)
    -> IResult<&str, Instruction>{
        let (rem,instr_str) = no_space(input)?;
        let (rem,_) = space(rem)?;
        let (num,sign) = sign(rem)?;
        let val = match sign{
            "+" => num.parse::<i32>().unwrap(),
            "-" => - (num.parse::<i32>().unwrap()),
            _   => -99999999
        };
        let instr = match instr_str{
            "nop" => Instruction::Nop(val),
            "jmp" => Instruction::Jmp(val),
            "acc" => Instruction::Acc(val),
            _     => Instruction::Nop(-999)
        };
        Ok((rem,instr))
}

fn parse() ->
Result<Program,Error>{
    let f = File::open("input_day_eight")?;
    let mut instructions = Vec::new();
    let file = BufReader::new(f);
    for line in file.lines(){
        let l = line.unwrap();
        let (_,instr) = instruction(&l).unwrap();
        instructions.push((instr,false));
    }
    let program = Program{instructions,current_instruction: 0, count: 0};
    Ok(program)
}

fn main(){
    let mut program = parse().unwrap();
    println!("value of the accumulator: {}",
             program.execute())
}
