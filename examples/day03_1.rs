use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = std::io::stdin().lines().map(|x| x.unwrap()).collect();
    let mut result = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let a: HashSet<char> = (chars[0..chars.len()/2]).iter().map(|x| *x).collect();
        let b: HashSet<char> = (chars[chars.len()/2..]).iter().map(|x| *x).collect();
        let c = *a.intersection(&b).next().unwrap();
        result += match c {
            'a'..='z' => (c as u32) - ('a' as u32) + 1,
            'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            _ => todo!(),
        };
    }
    println!("{:?}", result);
    Ok(())
}
