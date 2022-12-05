fn main() {
    let lines: Vec<String> = std::io::stdin().lines().map(|x| x.unwrap()).collect();
    let mut result = 0;
    for line in lines {
        let elves = line.split(",").map(|x| x.split("-").map(|s| s.parse().unwrap()).collect()).collect::<Vec<Vec<i32>>>();
        if elves[0][0] <= elves[1][0] && elves[0][1] >= elves[1][1]
        || elves[0][0] >= elves[1][0] && elves[0][1] <= elves[1][1] {
            result += 1;
        }
    }
    println!("{}", result);
}
