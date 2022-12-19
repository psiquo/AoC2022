use std::{env, fs};

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "input.pest"]
struct packet_parsers;

#[derive(Debug)]
#[derive(Clone)]
enum Entity {
    Num(u32),
    List(Vec<Entity>)
}

enum Ret {
    True,
    False,
    Cont
}

fn extract_list(packet : Pair<Rule>) -> Entity {
    match  packet.as_rule() {
        Rule::num => {
            return Entity::Num(packet.as_str().parse().expect(""));
        }
        Rule::list => {
            let mut ret : Vec<Entity> = Vec::new();

            for m in packet.into_inner() {
                ret.push(extract_list(m));
            }

            return Entity::List(ret);
        }
        
        _ => panic!("Unexpected rule {:?} ", packet.as_rule())
    }
}

fn extract_packets(input: &str) -> Vec<(Entity,Entity)> {
    let packet_list = packet_parsers::parse(Rule::input, input).expect("")
                                                                            .next()
                                                                            .unwrap();
    let mut ret : Vec<(Entity,Entity)> = Vec::new();

    for packet in packet_list.into_inner() {
        let mut iter = packet.into_inner();

        let p1 = extract_list(iter.next().unwrap());
        let p2 = extract_list(iter.next().unwrap());

        ret.push((p1,p2));

    }   

    return  ret;
}

fn compare_packets(packet: &(Entity,Entity)) -> Ret {
    //println!("Comparing {:?} and {:?}",packet.0,packet.1);
    match packet {
        (Entity::List(l1), Entity::List(l2)) => {

            for i in 0..l1.len().min(l2.len()) {
                match compare_packets(&(l1[i].clone(),l2[i].clone())) {
                    Ret::True => return Ret::True,
                    Ret::False => return Ret::False,
                    _ => ()
                }
            }
            
            if l1.len() == l2.len() {
                return  Ret::Cont;
            } else if l2.len() < l1.len() {
                return Ret::False;
            } else {
                return  Ret::True;
            }
        }

        (Entity::Num(n1), Entity::Num(n2)) => {
            if n1 < n2 {
                Ret::True
            } else if  n1 == n2 {
                Ret::Cont
            } else {
                Ret::False
            }
        }

        (Entity::Num(n1),Entity::List(l1)) => {
            return compare_packets(&(Entity::List(vec![Entity::Num(*n1)]),Entity::List(l1.clone())));
        }

        (Entity::List(l1),Entity::Num(n1)) => {
            return compare_packets(&(Entity::List(l1.clone()),Entity::List(vec![Entity::Num(*n1)])));
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut packets : Vec<(Entity,Entity)> = extract_packets(&input);

    let mut ret = 0;
    for (i,p) in packets.iter().enumerate() {
        let (p1,p2) = p;

        match compare_packets(p){
            Ret::True => {
                println!("Packet {} is in order",i);
                ret += i+1;
            }
            _ => println!("Packets {} is not in order",i)

        }

    }

    println!("{}",ret);

    let first_divider = Entity::List(vec![Entity::List(vec![Entity::Num(2)])]);
    let second_divider = Entity::List(vec![Entity::List(vec![Entity::Num(6)])]);

    let mut packet_list : Vec<Entity> = packets.iter().fold(vec![first_divider.clone(),second_divider.clone()], |mut acc,p| {acc.push(p.0.clone()); acc.push(p.1.clone()); acc});

    let mut index1 = 1;
    let mut index2 = 1;

    for i in packet_list {
        //println!("{:?}",i);
        index1 += match compare_packets(&(i.clone(),first_divider.clone())) {
            Ret::True => {
                1
            },
            _ => 0
        };
        index2 += match compare_packets(&(i.clone(),second_divider.clone())) {
            Ret::True => {
                1
            },
            _ => 0
        };
    }

    println!("Decoder key: {}*{}={}",index1,index2,index1 * index2)

}
