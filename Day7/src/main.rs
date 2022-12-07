use std::{fs,env, usize, collections::HashMap, iter::Peekable};
use regex::Regex;

fn get_dir_size(start : &str, fs : &HashMap<String, Vec<(&str,i128)>>) -> i128 {
    let mut size = 0;
    for (name,s) in fs.get(start).unwrap().iter() {
        if *s == -1 {
            let mut dst = String::from(start);
            dst.push_str("/");
            dst.push_str(name);
            size += get_dir_size(&dst, fs);
        } else {
            size += s;
        }
    }

    size
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        panic!("Must provide a filename as argument");
    }

    let regex_cd = Regex::new(r"^\$ cd (.*)$").unwrap();
    let regex_ls = Regex::new(r"^\$ ls.*$").unwrap();
    let regex_entry = Regex::new(r"([a-z0-9]+) (.*)$").unwrap();

    let input = fs::read_to_string(&args[1]).expect("Cannot read input file");
    let mut input_iter = input.split("\n").peekable();
    
    let mut curpath : Vec<&str> = Vec::new();

    let mut fs: HashMap<String, Vec<(&str,i128)>> = HashMap::new();
    fs.insert(String::from("/"), Vec::new());

    while input_iter.peek().is_some() {
        let line = input_iter.next().unwrap();
        
        if regex_cd.is_match(line) {

            println!("{:?}\n{}",curpath, line);

            let target_dir = regex_cd.captures(line).unwrap().get(1).unwrap().as_str();

            match target_dir {
                ".." => {curpath.pop();},
                t => curpath.push(t)
            };

            if curpath.is_empty() {
                curpath.push("/");
            }
        }

        if regex_ls.is_match(line){
            while input_iter.peek().is_some() && !input_iter.peek().unwrap().starts_with("$") {
                let captures = regex_entry.captures(input_iter.next().unwrap()).unwrap();
                let (tp, name) = (captures.get(1).unwrap().as_str(),captures.get(2).unwrap().as_str());
                let mut size = -1;

                if tp == "dir" {
                    let mut dest = curpath.join("/");
                    dest.push_str("/");
                    dest.push_str(name);
                    fs.insert(dest, Vec::new());
                } else {
                    size = tp.parse::<i128>().expect("Cannot parse");
                }

                fs.get_mut(&String::from(curpath.join("/"))).unwrap().push((name,size));
        
            }
        }
    }

    let mut tot = 0;
    for dname in fs.keys() {
        let tmp = get_dir_size(dname, &fs);
        println!("{}: {}",dname, tmp);
        if tmp < 100000 {
            tot += tmp;
        }
    }

    println!("{}",tot);

    let updatesize = 30000000;
    let unusedspace = 70000000 - get_dir_size("/", &fs);
    let mut min = get_dir_size("/", &fs);

    for dname in fs.keys() {
        let tmp = get_dir_size(dname, &fs);
        
        if updatesize <= (unusedspace+ tmp) && tmp < min  {
            min = tmp;
        }
    }

    println!("{}",min);

}



