use std::fs;

use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines = include_str!("improved-calibration.txt").lines();

    let numbers_in_lines =
        lines.map(|line| line.split(|c: char| !c.is_numeric()).collect::<String>());

    let numbers = numbers_in_lines
        .map(|numbers| numbers[0..1].to_string() + &numbers[numbers.len() - 1..])
        .map(|number| number.parse::<u32>().unwrap());

    let sum = numbers.sum::<u32>();
    println!("The sum is: {sum}");

    Ok(())
}

#[allow(dead_code)]
fn read_input() -> color_eyre::Result<String> {
    let path = "improved-calibration.txt";
    let input = fs::read_to_string(path).wrap_err(format!("reading {path}"))?;

    Ok(input)
}
