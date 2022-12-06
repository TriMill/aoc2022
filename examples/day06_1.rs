use std::collections::HashSet;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let chars: Vec<char> = line.chars().collect();
    for (i, window) in chars.windows(4).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(window.iter().map(|x| *x));
        if set.len() == 4 {
            println!("{}", i + 4);
            return
        }
    }
}
