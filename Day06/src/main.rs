use std::{env,fs, collections::VecDeque, ops::{Index, Range}};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut datapacket: Vec<char> = Vec::new();

    for (i,c) in input.trim().chars().enumerate() {
        if datapacket.contains(&c) {
            datapacket.drain((0 ..datapacket.iter().position(|e| *e == c).unwrap() + 1));
        }

        datapacket.push(c);

        if datapacket.len() == 14 {
            println!("{}",i+1);
            break;
        }
    }
}
