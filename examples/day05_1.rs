fn main() {
    let mut lines = std::io::stdin().lines();
    let mut crates = Vec::new();
    let mut line = lines.next().unwrap().unwrap();
    let crate_count = line.len()/4 + 1;
    for _ in 0..crate_count {
        crates.push(vec![]);
    }
    loop {
        if line.chars().nth(1).unwrap().is_numeric() {
            break;
        }
        for i in 0..crate_count {
            let c = line.chars().nth(4*i+1).unwrap();
            if c != ' ' {
                crates[i].insert(0, c);
            }
        }
        line = lines.next().unwrap().unwrap();
    }
    lines.next().unwrap().unwrap();
    while let Some(Ok(line)) = lines.next() {
        let mut parts = line.split(" ");
        let n: usize = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();
        for _ in 0..n {
            let c = crates[from-1].pop().unwrap();
            crates[to-1].push(c);
        }
    }

    for c in crates.iter_mut() {
        print!("{}", c.pop().unwrap());
    }
    println!();
    
}
