use std::{fs,env};

fn build_range(range_def : &str) -> (i32,i32) {
    if let [min,max] = range_def.split("-").collect::<Vec<&str>>()[..] {
        (min.trim().parse().expect("Cannot parse int"),max.trim().parse().expect("Cannot parse int"))
    } else {
        panic!("Invalid pattern detected");
    }
}

fn check_ranges_p1(r1: (i32,i32), r2: (i32,i32)) -> bool {
    (r1.0 <= r2.0 && r1.1 >= r2.1) || (r1.0 >= r2.0 && r1.1 <= r2.1)
}

fn check_ranges_p2(r1: (i32,i32), r2: (i32,i32)) -> bool {
    (r1.1 >= r2.0 && r1.0 <= r2.1) 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut count_p1 = 0;
    let mut count_p2 = 0;


    for x in input.trim().split("\n") {
        let (left, right) =  if let [left,right] = x.split(",").collect::<Vec<&str>>()[..] {(left,right)} else {panic!("Invalid pattern detected")};

        count_p1 += if check_ranges_p1(build_range(left), build_range(right)) {1} else {0};
        count_p2 += if check_ranges_p2(build_range(left), build_range(right)) {1} else {0};
    }

    println!("Overlap count p1: {}",count_p1);
    println!("Overlap count p2: {}",count_p2);

}
