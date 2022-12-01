fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::io::stdin().lines();
    let mut elves = Vec::new();
    let mut elf = 0;
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<i32>()?;
        }
    }
    elves.sort();
    elves.reverse();
    println!("{}", elves[0] + elves[1] + elves[2]);
    Ok(())
}
