use std::{env, fs};
use strum::IntoEnumIterator; 
use strum_macros::EnumIter;

#[derive(Debug,EnumIter)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
struct State {
    pos : (usize,usize),
    height: i32,
    len : u32
}

impl State {
    fn new(pos : (usize,usize),matr : Vec<Vec<(char,i32)>>) -> Self {
        let height = State::get_height(matr[pos.0][pos.1].0);
        State { pos, height, len : 0}
    }

    fn next_states(&self, matr : &mut Vec<Vec<(char,i32)>>) -> Vec<State> {
        let mut ret = Vec::<State>::new();

        for d in Dir::iter() {
            if(self.has_dir(&d,matr)){
                let mut new_state = self.clone();
                new_state.update_dir(&d, matr);

                let (x,y) = new_state.pos;

                if matr[x][y].1 == -1 {
                    matr[x][y].1 = self.len as i32;
                    ret.push(new_state);
                }
            }
        }

        ret
        
    }

    fn has_dir(&self, d : &Dir, matr : &mut Vec<Vec<(char,i32)>>) -> bool {
        let (x,y) = self.pos;
        match d {
            Dir::Up => x > 0 && (State::get_height(matr[x-1][y].0) - self.height) <= 1,
            Dir::Down => x < matr.len()-1  && (State::get_height(matr[x+1][y].0) - self.height) <= 1,
            Dir::Left => y > 1  && (State::get_height(matr[x][y-1].0) - self.height) <= 1,
            Dir::Right => y < matr[0].len() -1  && (State::get_height(matr[x][y+1].0) - self.height) <= 1
        }
    }

    fn update_dir(&mut self, d : &Dir, matr : &mut Vec<Vec<(char,i32)>>) {
        if !self.has_dir(d, matr) {
            panic!("Invalid Direction")
        }

        self.len += 1;

        match d {
            Dir::Up => self.pos.0 -= 1,
            Dir::Down => self.pos.0 += 1,
            Dir::Left => self.pos.1 -= 1,
            Dir::Right => self.pos.1 += 1
        }

        let (x,y) = self.pos;
        self.height = State::get_height(matr[x][y].0);

    }

    fn get_height(c : char) -> i32 {
        c as i32 - ('a' as i32)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.pos, self.len)
    }
}

fn matrix_min(mut m1 : Vec<Vec<(char,i32)>>, m2 : Vec<Vec<(char,i32)>>) -> Vec<Vec<(char,i32)>> {
    for i in 0..m1.len() {
        for j in 0..m1[0].len() {
            m1[i][j].1 = (m1[i][j].1).min(m2[i][j].1)
        }
    }

    return  m1;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut matrix = Vec::<Vec<(char,i32)>>::new();
    let mut start = (0,0);
    let mut end = (0,0);

    for (l,line) in input.trim().split("\n").enumerate() {
        let mut mat_line = Vec::<(char,i32)>::new();
        for (c,ch) in line.chars().enumerate() {
            if ch == 'S' {
                mat_line.push(('a',-1));
                start.0 = l;
                start.1 = c;
            } else if ch == 'E' {
                mat_line.push(('z',-1));
                end.0 = l;
                end.1 = c;
            } else {
                mat_line.push((ch,-1));
            }
        }
        matrix.push(mat_line);
    }

    let mut states = vec![State::new(start, matrix.clone())];
    let mut finished = Vec::<State>::new();

    while !states.is_empty() {
        let mut next_states = Vec::<State>::new();

        for state in states.iter() {
            next_states.append(&mut state.next_states(&mut matrix));
        }

        for i in 0..next_states.len() {
            if next_states[i].pos == end {
                println!("Found finish state (index {}): {}",i,next_states[i]);
                return;
            }
        }

        if !finished.is_empty() {
            return;
        }

        states = next_states;

        println!("{}",states.len());
    }

}
