use std::fs;
fn main() {
    let input = fs::read_to_string("./input/input1.txt").expect("Cannot read input file");
    let mut max : i32 = 0;
    let mut tot : i32 = 0;

    for s in input.trim().split("\n") {
        if s == "" {
            max = max.max(tot);
            tot = 0;
            continue;
        }

        tot += s.parse::<i32>().unwrap();
    }

    println!("{}",max);
}
