use regex::Regex;
use std::{fs,env};

struct CargoPlate<'a> {
    columns: Vec<Vec<&'a str>>
}

impl<'a> CargoPlate<'a> {
    fn new(column_count : usize) -> Self { 
        let mut columns : Vec<Vec<&str>> = Vec::new();

        for i in 0..column_count {
            columns.push(Vec::new());
        }
        
        CargoPlate { columns }
    }

    fn add_container(&mut self, cont : &'a str, index : usize) {
        if cont.trim() != "" {
            self.columns[index].push(cont);  
        } 
    }

    fn reverse_columns(&mut self){
        for i in 0..self.columns.len() {
            self.columns[i].pop();
            self.columns[i].reverse();
        }
    }
    fn cargo_move_times(&mut self, from : usize, to : usize, count : usize) {
        for _ in 0..count {
            self.cargo_move(from, to);
        }
    }

    fn cargo_move(&mut self, from : usize, to : usize){
        let cont = self.columns[from-1].pop().expect("Cannot move from empty column");
        self.columns[to-1].push(cont);
        
    }

    fn print(&self) {
        for (i,c) in self.columns.iter().enumerate() {
            println!("{}: {:?}",i+1,c);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        panic!("Must provide a filename and a columns count as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let crate_regex = Regex::new(r".{3} ?").unwrap();
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut match_inst = false;

    let columns = *(&args[2].parse::<usize>().expect("Invalid column number")); 
    let mut plate = CargoPlate::new(columns);
    
    println!("Columns: {}",columns);

    for line in input.trim_end().split("\n") {
        if line.trim() == "" {
            println!("Starting move instructions");
            plate.reverse_columns();
            plate.print();
            match_inst = true;
            continue;
        }

        if !match_inst {
            for (n, item) in crate_regex.find_iter(line).enumerate() {
                 plate.add_container(item.as_str(), n);
            }
        } else {
            for cap in move_regex.captures_iter(line) {
                println!("{} -> {} [{} times]",&cap[2],&cap[3],&cap[1]);
                plate.cargo_move_times(str::parse(&cap[2]).expect("Invalid conversion"), 
                                         str::parse(&cap[3]).expect("Invalid conversion"), 
                                      str::parse(&cap[1]).expect("Invalid conversion"));
                plate.print();
            }
        }
    }

   
    for c in plate.columns.iter() {
        if let Some(ch) = c.last() {
            print!("{}",ch.trim().chars().nth(1).expect(""));
        }    
    }
    println!("")
}
