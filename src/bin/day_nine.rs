use std::fs::File;
use std::io::{BufRead,BufReader,Error};

fn parse() ->
Result<Vec<i64>,Error>{
    let f = File::open("input_day_nine")?;
    let mut bunch = Vec::new();
    let file = BufReader::new(f);
    for line in file.lines(){
        bunch.push(line.unwrap()
                   .parse::<i64>()
                   .unwrap());
    }
    Ok(bunch)
}

fn is_valid(test: &i64, preamble: &[i64])
    -> bool {
    let mut valid = false;
    for a in preamble{
        for b in preamble{
            if a+b == *test {valid = true;}
        }
    }
    valid
}

fn main(){
    let bunch = parse().unwrap();
    let rem = &bunch[25..bunch.len()];
    let mut offset = 0;
    for elt in rem{
        if !is_valid(elt,
            &bunch[0+offset..25+offset]){
                println!("lol g trouvé {}",elt);
            }
        offset +=1;
    }
}
