use std::{fs,env, collections::HashSet};

fn are_adiacent(tail : &(i32,i32), head: &(i32,i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}
fn update_tail_pos(tail : &mut (i32,i32), head: &(i32,i32)) {
    if are_adiacent(tail, head) {
        return ();
    }


    if tail.0 == head.0 {
        tail.1 += if tail.1 < head.1 { 1 } else { -1 }
    } else if tail.1 == head.1 {
        tail.0 += if tail.0 < head.0 { 1 } else { -1 }
    } else if tail.0 < head.0 && tail.1 < head.1 {
        tail.0 += 1;
        tail.1 += 1;
    } else if tail.0 < head.0 && tail.1 > head.1 {
        tail.0 += 1;
        tail.1 -= 1;
    } else if tail.0 > head.0 && tail.1 > head.1 {
        tail.0 -= 1;
        tail.1 -= 1;
    } else if tail.0 > head.0 && tail.1 < head.1 {
        tail.0 -= 1;
        tail.1 += 1;
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut knots_coords = vec![(0,0);10];

    let mut tail_positions = HashSet::new();
    tail_positions.insert(knots_coords[8]);

    for dir in input.trim().split("\n") {
        let params = dir.split(" ").collect::<Vec<&str>>();
        let mov_count: i32 = params[1].parse().unwrap();
        
        println!("{}", dir);
        
        for _ in 0..mov_count {
            
            match params[0] {
                "U" => knots_coords[0].0 += 1,
                "D" => knots_coords[0].0 -= 1,
                "R" => knots_coords[0].1 += 1,
                "L" => knots_coords[0].1 -= 1,
                _ => panic!("Unexpected input value {}", params[0])
            }
            
            for i in 0..9 {
                let (shead, stail) = knots_coords.split_at_mut(i+1);
                update_tail_pos(&mut stail[0], shead.last().unwrap());
            }
            tail_positions.insert(knots_coords[9]);

            for (n,pos) in knots_coords.iter().enumerate() {
                println!("Knot {} at pos {:?}",n,pos);
            }
        }
    }

    println!("Different positions visited: {}", tail_positions.len());

}
