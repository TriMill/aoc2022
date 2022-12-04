fn main() {
    let lines = std::io::stdin().lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let result = lines.chunks(3)
        .map(|chunk| 
            chunk[0].chars()
                .filter(|c| chunk[1].contains(*c) && chunk[2].contains(*c))
                .next().unwrap()
        ).map(|c| match c {
            'a'..='z' => (c as u32) - ('a' as u32) + 1,
            'A'..='Z' => (c as u32) - ('A' as u32) + 27,
            _ => unreachable!()
        }).reduce(|a, b| a + b)
        .unwrap();
    println!("{}", result);
}
