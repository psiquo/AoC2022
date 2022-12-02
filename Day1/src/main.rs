use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut elf_calories : Vec<i32> = Vec::new();
    let mut tot : i32 = 0;

    for s in input.trim().split("\n") {
        if s == "" {
            elf_calories.push(tot);
            tot = 0;
            continue;
        }

        tot += s.parse::<i32>().unwrap();
    }

    elf_calories.sort_by(|a,b| b.cmp(a));
    println!("Max: {}",elf_calories.iter().max().expect("Cannot find max"));
    println!("Sum of the top three: {}", &elf_calories[0..3].iter().sum::<i32>());
}   
