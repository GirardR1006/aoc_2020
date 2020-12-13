extern crate nom;

use std::io::{BufRead,BufReader,Error};
use std::fs::File;
use std::collections::HashMap;
use nom::IResult;
use nom::bytes::complete::{take_while1, tag};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::branch::alt;

type Weight = usize;
type Id = usize;
type Neighbours = Vec<(Id,Weight)>;

// a simple graph structure based
// on adjacency lists
#[derive(Debug)]
struct Node{
    id: Id,
    preds : Neighbours,
    succs : Neighbours
}
impl Node{
    fn new(id:Id,
           preds: Neighbours,
           succs: Neighbours) -> Node{
        Node{id, preds, succs}
    }
}
#[derive(Debug)]
struct Graph{
    count: usize,
    colours: HashMap<String,usize>,
    nodes: Vec<Node>
}

impl Graph {
    fn new() -> Graph{
        Graph {count: 0, colours: HashMap::new(),
        nodes: Vec::new()}
    }
    fn add(self: &mut Graph,
           colour: &String,
           preds: Neighbours,
           succs: Neighbours){
        match self.colours.get(colour){
            Some(_) => (), //there is already an id
            //associated to this colour
            None => { //no existing colour: add the node
                let new_id = self.count;
                self.colours.insert(colour.to_string(),new_id);
                let n = Node::new(new_id,preds,succs);
                self.nodes.push(n);
                self.count += 1;
            }
        };
    }
    fn mem(self: &Graph, id: usize) -> bool{
        id < self.count
    }
    fn find_id_by_colour (self: &mut Graph, colour: String)
        -> Option<&usize>{
            self.colours.get(&colour)
        }
    fn find_node_by_id (self: &mut Graph,id : &usize)
        -> Option<&mut Node>{
            self.nodes.iter_mut()
                .filter(|n| n.id == *id)
                .next()
        }
    fn find_node_by_id_immut (self: &Graph,id : &usize)
        -> Option<&Node>{
            self.nodes.iter()
                .filter(|n| n.id == *id)
                .next()
        }
    fn find_node_by_colour (self: &Graph,colour : String)
        -> Option<&Node>{
            let id = self.colours.get(&colour).unwrap();
            self.nodes.iter()
                .filter(|n| n.id == *id)
                .next()
        }
    //add edge to a graph, require all nodes to be already here
    fn add_e(self: &mut Graph,
             id: usize, pred_id: usize,
             w: usize){
        match (self.mem(id),self.mem(pred_id)){
            (true,true) => {
                self.find_node_by_id(&id)
                    .unwrap()
                    .preds
                    .push((pred_id,w));
                self.find_node_by_id(&pred_id)
                    .unwrap()
                    .succs
                    .push((id,w));
            },
            (_,_) => ()
        }
    }
    fn get_preds(self: &Graph, node: &Node)
    -> Vec<&Node>{
        let ids : Vec<usize> = node.preds.iter().map(|x| x.0).collect();
        let preds = ids
            .iter()
            .map(|id| self.find_node_by_id_immut(id).unwrap())
            .collect();
        preds
    }
}

//basic steps:
//split on the string "contains"
//create node for leftside part of this sequence
//for right side, check whether there is "no other"
//in the sequence; if not, build succs
fn no_space(s: &str)
    -> IResult<&str,&str>{
        take_while1(|c| c !=' ')(s)
}
fn space(s: &str)
    -> IResult<&str,&str>{
        take_while1(|c| c ==' ')(s)
}
//parser for left sequence
fn colour(input: &str)
    -> IResult<&str, String>{
        let (rem,adj) = no_space(input)?;
        let (rem,_) = space(rem)?;
        let (rem,col) = no_space(rem)?;
        let full_colour = [adj,col].concat();
        Ok((rem,full_colour.to_string()))
    }
//parser for right sequence: everything before final .,
//either contains "no other input" and thus there is no need to add
//it to the graph, either it contains a sequence
//describing the containing capacity of several bags
fn bags(input: &str)
    -> IResult<&str, Vec<(String,usize)>>{
        let dot = take_while1(|c| c !='.');
        let (_,input) = dot(input)?;
        alt((map(tag(" no other bags"), |_| Vec::new()),
        separated_list1(tag(","),contain_seq)
        ))(input)
}
//parser for one bag: [space]N[space][colour]
fn contain_seq(input: &str)
    -> IResult<&str, (String, usize)>{
        let (rem,_) = space(input)?;
        let (rem,number) = no_space(rem)?;
        let (rem,_) = space(rem)?;
        let (rem,colour) = colour(rem)?;
        let (rem,_) = take_while1(|c| c !=',')(rem)?;
        Ok((rem,(colour,number.parse::<usize>().unwrap())))
    }

fn parse() -> Result<Graph,Error>{
    let f = File::open("input_day_seven_test")?;
    let file = BufReader::new(f);
    let mut g = Graph::new();
    for line in file.lines(){
        let l = line.unwrap();
        let v : Vec<&str> = l
            .split("contain")
            .collect();
        //first element is a colour bag
        let (_,colour) = colour(v[0]).unwrap();
        g.add(&colour, Vec::new(), Vec::new());
        let base_id = *g.find_id_by_colour(colour).unwrap();
        //second element is a maybe bag sequence
        let (_,res_bagseq) = bags(v[1]).unwrap();
        for (target_colour,capacity) in res_bagseq.iter(){
            g.add(target_colour, Vec::new(), Vec::new());
            let target_id = *g.find_id_by_colour(
                target_colour.to_string()).unwrap();
            g.add_e(base_id, target_id, *capacity)
        }
    }
    Ok(g)
}

//parse the golden colour, and count all
//predecessors and their predecessors
fn aux(graph: &Graph, n: &Node) -> usize{
    match graph.get_preds(n).len(){
        0 => 1,
        _  => graph
            .get_preds(n)
            .iter()
            .fold(0,|acc,x| acc + aux(graph, x))
    }
}
fn count_bags(graph: &mut Graph) -> usize{
    let n = graph
        .find_node_by_colour(String::from("shinygold"))
        .unwrap();
    aux(graph, n)
}


fn main(){
    println!("Day seven!");
    let mut g = parse().unwrap();
    println!("Number of preds {}",count_bags(&mut g));
}
