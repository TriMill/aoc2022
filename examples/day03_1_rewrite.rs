fn main() {
    println!("{}", std::io::stdin().lines()
        .map(|x| x.unwrap())
        .map(|line| (
            line.chars().take(line.len() / 2).collect::<Vec<char>>(), 
            line.chars().skip(line.len() / 2).collect::<Vec<char>>()
        )).map(|(a, b)| *a.iter().filter(|c| b.contains(c)).next().unwrap())
        .map(|c| match c {
            'a'..='z' => (c as u32) - ('a' as u32) + 1,
            'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            _ => unreachable!()
        }).reduce(|a, b| a + b)
        .unwrap());
}
