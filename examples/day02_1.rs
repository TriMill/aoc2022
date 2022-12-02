fn score(a: char, b: char) -> i32 {
    match (a, b) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => todo!()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = std::io::stdin().lines().map(|x| x.unwrap()).collect();
    let mut total = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        chars.next();
        let b = chars.next().unwrap();
        total += score(a, b);
    }
    println!("{}", total);
    Ok(())
}
