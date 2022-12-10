use std::{fs,env};

fn crt_draw(screen : &mut Vec<char>, cycles : i32, x : i32) {
    let row_pos = (cycles-1) % 40;

    if (x-1..x+2).contains(&row_pos) {
        screen.push('#');
    } else {
        screen.push('.');
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    
    let mut cycles = 1;
    let mut x = 1;
    let mut total = 0;
    let mut crt : Vec<char> = Vec::new();

    for line in input.trim().split("\n") {        
        let params = line.split(" ").collect::<Vec<&str>>();

        match params[0] {
            "noop" => {
                crt_draw(&mut crt, cycles, x);
                cycles += 1
            },
            "addx" => {
                crt_draw(&mut crt, cycles, x);
                cycles += 1;

                if (cycles + 1 - 20) % 40 == 0 {
                    println!("[IN ADD] Passed {} cycles the signal strength is {} (value of x is {})", cycles + 1, (cycles + 1) * x, x);
                    total += (cycles + 1) * x;
                }
                
                crt_draw(&mut crt, cycles, x);

                cycles += 1;
                x += params[1].parse::<i32>().unwrap();

            },
            _ => panic!("Invalid command {}", params[0])
        }

        if (cycles - 20) % 40 == 0 {
            println!("Passed {} cycles the signal strength is {} (value of x is {})", cycles, cycles * x,x);
            total += cycles * x;
        }
    }

    println!("Cumulative signal strength: {}",total);

    for x in crt.chunks(40) {
        println!("{}",x.iter().collect::<String>());
    }
}
