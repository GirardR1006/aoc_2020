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
// change the instruction at index id and reset the program
// execution state
fn change_instr(instr:&Instruction)->Instruction{
    match instr{
        Instruction::Acc(i) =>{
            Instruction::Acc(*i)
        },
        Instruction::Jmp(i) =>{
            Instruction::Nop(*i)
        },
        Instruction::Nop(i) =>{
            Instruction::Jmp(*i)
        }
    }
}
#[derive(Debug)]
struct Program{
    instructions: Vec<(Instruction,bool)>,
    current_instruction: i32,
    count: i32
}
impl Program{
    fn find_terminal_state(self: &mut Program) -> i32{
        let mut id_to_mod = 0;
        let mut acc = 0;
        let mut terminated = false;
        println!("Initial program {:?}",self);
        while !terminated{
            let res = self.execute();
            acc = res.0;
            terminated = res.2;
            self.mutate(id_to_mod);
            id_to_mod += 1;
        }
        println!("Found instruction to modify at index: {}", id_to_mod);
        acc
    }
    fn mutate(self: &mut Program, id: usize) {
        let (cur_instr,_) = &self.instructions[id];
        self.instructions[id] = (change_instr(cur_instr),false);
        //revert change made previously
        if id > 0{
            let (prev_instr,_) = &self.instructions[id-1];
            self.instructions[id-1] = (change_instr(prev_instr),false)
        }
        // resetting all visited instructions
        for (_,switch) in &mut self.instructions {
            *switch = false;
        };
        self.count = 0;
        self.current_instruction = 0;

    }
    fn execute(self: &mut Program) -> (i32,bool,bool){
        let mut infinite = false;
        let mut terminated = false;
        while !infinite && !terminated {
            let num_instructions = self.instructions.len().clone();
            let cur_idx = self.current_instruction as usize;
            if cur_idx >= num_instructions {terminated = true}
            else{
                let (cur_instr,switch) = &mut self.instructions[cur_idx];
                if *switch {infinite = true}
                else{
                    match cur_instr {
                        Instruction::Acc(i) =>{
                            *switch = true;
                            self.count += *i;
                            self.current_instruction += 1
                        },
                        Instruction::Jmp(i) =>{
                            *switch = true;
                            self.current_instruction += *i
                        },
                        Instruction::Nop(_i) =>{
                            *switch = true;
                            self.current_instruction += 1
                        }
                    }
                }
            };
        }
        (self.count,infinite,terminated)
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
    println!("value of the accumulator after mutation: {}",
             program.find_terminal_state())
}
