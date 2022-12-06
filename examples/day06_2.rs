use std::collections::HashSet;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let chars: Vec<char> = line.chars().collect();
    for (i, window) in chars.windows(14).enumerate() {
        let set: HashSet<char> = HashSet::from_iter(window.iter().map(|x| *x));
        if set.len() == 14 {
            println!("{}", i + 14);
            return
        }
    }
}
