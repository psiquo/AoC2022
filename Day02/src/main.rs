use core::panic;
use std::{env, fs};

#[derive(Debug)]
enum Output {
    Win,
    Loss,
    Draw
}

impl Output {
    fn parse(outcome : &str) -> Output{
        match outcome {
            "Z" => Output::Win,
            "Y" => Output::Draw,
            "X" => Output::Loss,
            _ => panic!("Invalid outcome: {}",outcome)        
        }
    }

    fn evaluate(&self) -> i32{
        match self {
            Output::Win => 6,
            Output::Draw => 3,
            Output::Loss => 0
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor
}

impl Hand {
    fn parse(hand: &str) -> Hand{
        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
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

    fn fix_player(elf: Hand, outcome: Output) -> Hand {
        match outcome {
            Output::Win => match elf {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock
            },
            Output::Draw => elf,
            Output::Loss => match elf {
                Hand::Rock => Hand::Scissor,
                Hand::Paper => Hand::Rock,
                Hand::Scissor => Hand::Paper
            }
        }
    }

}

fn convert_to_elf_hand(hand: &str) -> &str{
    match hand {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!("Invalid LHS, cannot convert {} to elf lingo",hand)
    }
}
fn evaulate_round_p1(hand: &str, score: &mut i32){
    let (elf, player) = (hand.split(" ").collect::<Vec<&str>>()[0], hand.split(" ").collect::<Vec<&str>>()[1]);

    let mut outcome_score = match (Hand::parse(elf), Hand::parse(convert_to_elf_hand(player))) {
        (Hand::Rock,Hand::Paper) | (Hand::Paper,Hand::Scissor) | (Hand::Scissor, Hand::Rock) => {Output::Win},
        (x,y) => if x == y { Output::Draw} else { Output::Loss}
    }.evaluate();

    outcome_score += Hand::parse(convert_to_elf_hand(player)).evaluate();

    *score += outcome_score;

    println!("Hand ({},{}) lead to a score of {}", elf, player, outcome_score);
}

fn evaulate_round_p2(hand: &str, score: &mut i32) -> () {
    let (elf, outcome) = (hand.split(" ").collect::<Vec<&str>>()[0], hand.split(" ").collect::<Vec<&str>>()[1]);
    let player = Hand::fix_player(Hand::parse(elf), Output::parse(outcome));
    
    let hand_score = player.evaluate() + Output::parse(outcome).evaluate();
    *score += hand_score;

    println!("Output of hand ({},{}) is {} (player set to {:?} and outcome to {:?})", elf, outcome, hand_score,player,Output::parse(outcome));
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut total_p1 = 0;
    let mut total_p2 = 0;

    for entry in input.trim().split("\n") {
        evaulate_round_p1(entry, &mut total_p1);
        evaulate_round_p2(entry, &mut total_p2);
    }

    println!("Total as problem one rules: {}",total_p1);
    println!("Total as problem two rules: {}",total_p2);
}
