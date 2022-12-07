use std::{collections::{HashMap, HashSet}};

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap()).peekable();
    let mut files: HashMap<String, usize> = HashMap::new();
    let mut dirs: HashSet<String> = HashSet::new();
    let mut cd: Vec<String> = Vec::new();
    while let Some(line) = lines.next() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[1] {
            "cd" => {
                match parts[2] {
                    ".." => { cd.pop(); },
                    "/" => cd = Vec::new(),
                    x => cd.push(x.to_owned()),
                }
                dirs.insert(cd.join("/"));
            }
            "ls" => while let Some(line) = lines.peek() {
                if line.starts_with("$") {
                    break
                }
                let line = lines.next().unwrap();
                let parts: Vec<&str> = line.split(" ").collect();
                if parts[0] != "dir" {
                    let size: usize = parts[0].parse().unwrap();
                    let name = parts[1].to_owned();
                    let mut f = cd.clone();
                    f.push(name);                    
                    files.insert(f.join("/"), size);
                }
            }
            _ => todo!()
        }
    }
    let mut sizes: HashMap<String, usize> = HashMap::new();
    for dir in dirs {
        let mut size = 0;
        for file in &files {
            if file.0.starts_with(&dir) {
                size += file.1;
            }
        }
        sizes.insert(dir, size);
    }
    let total: usize = sizes.iter().filter(|(_, v)| **v <= 100000).map(|(_, v)| v).sum();
    println!("{}", total);
}
