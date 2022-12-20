use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut paths : Vec<Vec<(i32,i32)>> = Vec::new();

    let mut min_height = -1;
    let mut max_height = -1;
    let mut min_width = -1;
    let mut max_width = -1;

    for line in input.trim().split("\n") {
        let mut path : Vec<(i32,i32)> = Vec::new();

        for node in line.split(" -> "){
            let mut node_tuple = (0,0);
            
            let mut coords = node.split(",");

            node_tuple.1 = coords.next().unwrap().parse().expect("");
            node_tuple.0 = coords.next().unwrap().parse().expect("");
            path.push(node_tuple);
            
            min_height = if min_height == -1 {
                node_tuple.0
            } else {
                min_height.min(node_tuple.0)
            };

            max_height = if max_height == -1 {
                node_tuple.0
            } else {
                max_height.max(node_tuple.0)
            };

            min_width = if min_width == -1 {
                node_tuple.1
            } else {
                min_width.min(node_tuple.1)
            };

            max_width = if max_width == -1 {
                node_tuple.1
            } else {
                max_width.max(node_tuple.1)
            };

        }

        paths.push(path);

    }

    //println!("MinH: {}\nMaxH: {}\nMinW: {}\nMaxW: {}",min_height, max_height,min_width,max_width);

    let vec_width = max_width - min_width;
    let vec_height = max_height - min_height;

    //println!("Creating vector of size {}x{}",vec_height,vec_width);

    let mut matr = vec![vec!["."; (vec_width + 1 + (300)).try_into().unwrap()];(max_height + 1 + 2).try_into().unwrap()];

    for i in 0..matr[0].len() {
        matr.last_mut().unwrap()[i] = "#";
    }

    insert_rocks(& paths, &mut matr,min_width - 150, 0);

    print_matr(&matr);

    let mut count = 0;
    while !insert_sand(&mut matr, (0,500 + 150 - min_width)) {
        //print_matr(&mut matr);
        count += 1;
    }
    print_matr(&mut matr);


    println!("{} grain of sands nedded before they start dropping in the abyss",count);

}

fn insert_sand(matr: &mut Vec<Vec<&str>>, start: (i32, i32)) -> bool {
    let mut prev = start;
    matr[prev.0 as usize][prev.1 as usize] = "o";
    while true {
            //print_matr(matr);
            // /println!("");
            
            // if prev.0 + 1 >= matr.len().try_into().unwrap() ||
            //    prev.1 + 1 >= matr[0].len().try_into().unwrap() ||
            //    prev.1 - 1 < 0 {
            //     matr[prev.0 as usize][prev.1 as usize] = ".";
            //     return true;
            // }
            

            if matr[(prev.0 + 1) as usize][prev.1 as usize] == "." {
                matr[prev.0 as usize][prev.1 as usize] = ".";
                prev.0 += 1;
                matr[prev.0 as usize][prev.1 as usize] = "o";
                continue;
            }

            if matr[(prev.0 + 1) as usize][(prev.1 - 1) as usize] == "." {
                matr[prev.0 as usize][prev.1 as usize] = ".";
                prev.0 += 1;
                prev.1 -= 1;
                matr[prev.0 as usize][prev.1 as usize] = "o";
                continue;
            }

            if matr[(prev.0 + 1) as usize][(prev.1 + 1) as usize] == "." {
                matr[prev.0 as usize][prev.1 as usize] = ".";
                prev.0 += 1;
                prev.1 += 1;
                matr[prev.0 as usize][prev.1 as usize] = "o";
                continue;
            }
            
            println!("Prev: {:?}",prev);
            return prev == start;
            
    }

    return  true;
}

fn print_matr(matr: &Vec<Vec<&str>>) {
    for line in matr.iter() {
        for elem in line.iter() {
            print!("{}",elem);
        }
        println!("");
    }
}

fn insert_rocks(paths: &Vec<Vec<(i32, i32)>>, matr: &mut Vec<Vec<&str>>, lower_width : i32, lower_height : i32) {
    for path in paths.iter() {
        let mut prev = path[0];
        
        //println!("Prev: {:?}",prev);
        
        for coords in path[1..].iter() {
            let cur = coords;

            //println!("Cur {:?}", cur);
            while prev != *cur {
                //println!("Accessing matr[{}][{}]", (prev.0 - lower_height) as usize, (prev.1 - lower_width) as usize);
                matr[(prev.0 - lower_height) as usize][(prev.1 - lower_width) as usize] = "#";

                if prev.0 == cur.0 {
                    prev.1 += if prev.1 < cur.1 { 1 } else {-1};
                } else {
                    prev.0 += if prev.0 < cur.0 { 1 } else {-1};
                }

                //println!("Prev: {:?}",prev);
            }

            matr[(prev.0 - lower_height) as usize][(prev.1 - lower_width) as usize] = "#";
        }
    }
}
