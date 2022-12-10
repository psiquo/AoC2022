use core::panic;
use std::{env, fs, ops::Index, fmt::Display};

struct Items<T> {
    item_list : Vec<T>,
}

impl<T : Eq + Display> Items<T> {
    fn new(item_list : Vec<T>) -> Items<T> {
        Items {item_list}
    }

    fn get_priority(&self, item : T) -> usize {
        if let Some(n) = self.item_list.iter().position(|x| *x == item) {
            n + 1
        } else {
            panic!("No such item in the list: {}", item);
        }
    }
}

fn get_badges(rucksack_list : Vec<&str>, mut acc :Vec<char>) -> Vec<char> {
    match rucksack_list[..] {
        [] => acc,
        [a,b,c, ..] => {
            let badge = a.chars().filter(| ch| b.contains(*ch) && c.contains(*ch)).collect::<Vec<char>>()[0];
            acc.push(badge);
            get_badges(rucksack_list[3..].to_vec(), acc)
        }
        _ => panic!("Invalid Pattern")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut letters : Vec<char> = ('a'..='z').collect();
    letters.append(&mut ('A'..='Z').collect());

    let rucksack = Items::new(letters);

    let mut priority : usize = 0;

    for entry in input.trim().split("\n") {
        let (first,second) = (&entry[..entry.len()/2],&entry[entry.len()/2..]);
        let misplaced : char  = first.chars().filter(|a| second.contains(*a)).collect::<Vec<char>>()[0];
        priority += rucksack.get_priority(misplaced);
    }

    let rucksacks : Vec<&str> = input.trim().split("\n").collect();
    let mut badge_priority: usize = 0;

    for badge in get_badges(rucksacks, Vec::new()).iter() {
        badge_priority += rucksack.get_priority(*badge);
    }
    println!("Total priority: {}",priority);
    println!("Badge priority: {}",badge_priority);

}
