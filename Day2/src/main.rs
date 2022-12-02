use core::panic;
use std::{env, fs};

enum Output {
    Win,
    Loss,
    Draw
}

impl Output {
    fn evaluate(&self) -> i32{
        match self {
            Output::Win => 6,
            Output::Draw => 3,
            Output::Loss => 0
        }
    }
}

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor
}

impl Hand {
    fn parse_elf(hand: &str) -> Hand{
        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
            _ => panic!("Invalid elf hand: {}", hand)
        }
    }

    fn parse_player(hand: &str) -> Hand{
        match hand {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissor,
            _ => panic!("Invalid elf hand: {}", hand)
        }
    }

    fn evaluate(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3
        }
    }

}

fn evaulate_round(hands: &str, score: &mut i32){
    let (elf, player) = (hands.split(" ").collect::<Vec<&str>>()[0], hands.split(" ").collect::<Vec<&str>>()[1]);

    let mut outcome_score = match (Hand::parse_elf(elf), Hand::parse_player(player)) {
        (Hand::Rock,Hand::Paper) | (Hand::Paper,Hand::Scissor) | (Hand::Scissor, Hand::Rock) => {Output::Win},
        (x,y) => if x == y { Output::Draw} else { Output::Loss}
    }.evaluate();

    outcome_score += Hand::parse_player(player).evaluate();

    *score += outcome_score;

    println!("Hand ({},{}) lead to a score of {}", elf, player, outcome_score);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut total = 0;

    for entry in input.trim().split("\n") {
        evaulate_round(entry, &mut total);
    }

    println!("{}",total);
}
