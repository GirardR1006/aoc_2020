use std::io::{BufRead,BufReader,Error};
use std::fs::File;

struct ForestMap {
    width : usize,
    height: usize,
    topo : Vec<String>
}

fn read_input() -> Result<ForestMap,Error>{
    let f = File::open("input_day_three_test")?;
    let mut topo = Vec::new();
    let file = BufReader::new(f);
    for line in file.lines(){
        topo.push(line.unwrap());
    }
    let map = ForestMap {
        width : topo[0].len(),
        height : topo.len(),
        topo : topo
    };
    Ok(map)
}

fn weeeee(map:&ForestMap, right: usize,down: usize)
    -> usize{
    let mut trees = 0;
    let mut x = 0;
    for y in (down..map.height).step_by(down){
        x = (x + right)%map.width;
        let pouet = map.topo[y]
            .chars()
            .nth(x%map.width)
            .unwrap();
        if pouet == '#'{
                trees += 1;
            }
    }
    trees
}


fn main() {
    println!("Day three!");
    let map = read_input().unwrap();
    let trees = weeeee(&map, 3, 1);
    println!("Total number of trees met during descend: {}",trees);
    let slopes = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let treess = slopes
        .iter()
        .fold(1, |acc, &t| acc*weeeee(&map, t.0, t.1));
    println!("Total number of trees met during descends:
    {}",treess);
}
