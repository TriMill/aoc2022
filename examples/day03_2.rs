use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = std::io::stdin().lines().map(|x| x.unwrap()).collect();
    let mut result = 0;
    for window in lines.chunks(3) {
        let a: HashSet<char> = (window[0]).chars().collect();
        let b: HashSet<char> = (window[1]).chars().collect();
        let c: HashSet<char> = (window[2]).chars().collect();
        let d: HashSet<char> = a.intersection(&b).map(|x| *x).collect();
        let mut e = d.intersection(&c);
        result += match *e.next().unwrap() {
            w @ 'a'..='z' => (w as u32) - ('a' as u32) + 1,
            w @ 'A'..='Z' => (w as u32) - ('A' as u32) + 27,
            _ => todo!(),
        };
    }
    println!("{:?}", result);
    Ok(())
}
