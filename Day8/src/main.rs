use std::{env, fs};

fn check_line(line : &Vec<u32>, index : usize) -> bool {
    line[..index].iter().any(|n| n >= line.get(index).unwrap()) &&
    line[index+1..].iter().any(|n| n >= line.get(index).unwrap())
}

fn get_column(matrix : &Vec<Vec<u32>>, col_index : usize) -> Vec<u32> {
    let mut tmp = Vec::new();

    for line in matrix {
        tmp.push(*line.get(col_index).unwrap());
    }

    tmp
}

fn get_scenic_score(matrix : &Vec<Vec<u32>>, row : usize, column : usize) -> i32 {
    let (r1, r2) = get_view_distance(&matrix[row], column);
    let (c1,c2) = get_view_distance(&get_column(matrix, column),row);

    r1 * r2 * c1 * c2
}

fn get_view_distance(row: &Vec<u32>, index: usize) -> (i32,i32) {
    let mut c1 = 0;

    for &el in row[..index].iter().rev() {
        c1 += 1;

        if el >= *row.get(index).unwrap() {
            break;
        }
    }

    let mut c2 = 0;

    for &el in row[index+1..].iter() {
        c2 += 1;

        if el >= *row.get(index).unwrap() {
            break;
        }
    }

    (c1,c2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");

    let mut matrix : Vec<Vec<u32>> = Vec::new();

    for line in input.trim().split("\n") {
        let mut tmp = Vec::new();
        for c in line.chars() {
            tmp.push(c.to_digit(10).unwrap());
        }

        matrix.push(tmp);
    }
    println!("Tree matrix:");
/* 
    for line in matrix.iter() {
        println!("{:?}",line);
    }
*/
    let height = matrix.len();
    let width = matrix[0].len();

    println!("Height: {} Width: {}",height,width);

    let mut count = height * 2 + (width - 2) * 2;
    let mut max_scenic = 0;

    for i in 1..(height-1) {
        for j in 1..(width-1){
            if !check_line(matrix.get(i).unwrap(), j) || 
               !check_line(&get_column(&matrix,j), i) {
                count += 1;
            }

            max_scenic = max_scenic.max(get_scenic_score(&matrix, i, j));
        }
    }

    println!("Visibility count: {}",count);
    println!("Max scenic score: {}",max_scenic);
}
