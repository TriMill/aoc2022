fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::io::stdin().lines();
    let mut max = 0;
    let mut elf = 0;
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            max = max.max(elf);
            elf = 0;
        } else {
            elf += line.parse::<i32>()?;
        }
    }
    println!("{}", max);
    Ok(())
}
