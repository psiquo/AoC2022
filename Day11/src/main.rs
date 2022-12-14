use std::{env, fs, collections::HashMap, cell::RefCell};

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use itertools::Itertools;

#[derive(Parser)]
#[grammar = "monkey.pest"]
struct monkey_parser;


struct Monkey {
    id : u8,
    items : Vec<u32>,
    update :  Box<dyn Fn(u32)->u32>,
    divisor : u32,
    true_br : u8,
    false_br : u8,
    counted_items: u32,
}

impl Monkey {
    fn new() -> Self {
        return Monkey { 
            id: 0, 
            items: Vec::new(), 
            update: Box::new(|x: u32| x), 
            divisor: 1, 
            true_br: 0, 
            false_br: 0,
            counted_items : 0,
        }
    }

    fn round(&mut self, monkeys: &HashMap<u8,RefCell<Monkey>>){
        for item in self.items.iter() {
            self.counted_items += 1;

            let worry = (self.update)(*item);
            let branch = if worry % self.divisor == 0 {
               &self.true_br
            } else {
                &self.false_br
            };

            monkeys.get(branch).unwrap().borrow_mut().items.push(worry);
        }

        self.items.clear();
    }
}

fn extract_monkeys(input: &str) -> HashMap<u8,RefCell<Monkey>> {
    
    let monkeys = monkey_parser::parse(Rule::monkeys, input).expect("Cannot parse file")
                                                                         .next()
                                                                         .unwrap();
    let mut monkey_map = HashMap::<u8,RefCell<Monkey>>::new();

    for monkey in monkeys.into_inner() {
        let mut m = Monkey::new();

        for param in monkey.into_inner() {
            match param.as_rule() {
                Rule::monkey_decl => {
                    let id = param.into_inner().next().unwrap().as_str();
                    println!("Monkey has id {}", id);
                    m.id = id.parse::<u8>().unwrap();
                },
                Rule::items => {
                    print!("Monkey has items: ");
                    for item in param.into_inner() {
                        let i = item.as_str().parse::<u32>().unwrap();
                        print!("{} ",i);
                        m.items.push(i);
                    }
                    println!("");
                },

                Rule::operation => {
                    let mut op_iter = param.into_inner();
                    let op = op_iter.next().unwrap().as_str();
                    let arg = op_iter.next().unwrap().as_str();
                    
                    m.update = 
                        match op {
                        "*" => if arg == "old" {
                            Box::new(|x| x * x)
                        } else {
                            let y =arg.parse::<u32>().unwrap();
                            println!("Arg is: {}",y);
                            Box::new((|y| move |x| x * y)(y) )
                        },
                        "+" => if arg == "old" {
                            Box::new(|x| x + x)
                        } else {
                            let y =arg.parse::<u32>().unwrap();
                            println!("Arg is: {}",y);
                            Box::new((|y| move |x| x + y)(y) )
                        },
                        _ => panic!("Invalid operation")
                    };

                    println!("Monkey updates old with old {} {}",op,arg);
                },

                Rule::test => {
                    let divisor = param.into_inner().next()
                                                         .unwrap()
                                                         .as_str()
                                                         .parse::<u32>()
                                                         .unwrap();
                    println!("Monkey checks if worry is divisible by {}", divisor);
                    m.divisor = divisor;
                }

                Rule::cond => {
                    let mut cond_iter = param.into_inner();
                    let cond_value = cond_iter.next().unwrap().as_str();
                    let pass_to = cond_iter.next().unwrap()
                                                        .as_str()
                                                        .parse::<u8>()
                                                        .unwrap();
                    println!("If worry is {}divisble by {} the monkey pass the item to {}", 
                        if cond_value == "true" {""} else {"not "},
                        m.divisor,
                        pass_to
                    );

                    if cond_value == "true" {
                        m.true_br = pass_to;
                    } else {
                        m.false_br = pass_to;
                    }

                }

                _ => println!("Different rule")
            }
        }

        monkey_map.insert(m.id, RefCell::new(m));
    }

    return  monkey_map;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut monkey_map = extract_monkeys(&input);
    let mut keys : Vec<u8> = monkey_map.keys().map(|x| *x).collect();
    keys.sort();

    for round in 0..20 {
        println!("ROUND {}",round);
        for m in keys.iter() {
            let monkey = monkey_map.get(m).unwrap();
            monkey.borrow_mut().round(&monkey_map);
        }

        for m in keys.iter() {
            let m = monkey_map.get(m).unwrap().borrow();
            println!("Monkey {} is holding items {:?}",m.id,m.items);
        }
    }

    println!("Monkey business: {}", monkey_map.values().map(|x| x.borrow().counted_items).sorted_by(|a,b| b.cmp(a)).take(2).fold(1, |a,b| a * b) );


}
