use std::fs;

use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("improved-calibration.txt");
    let mut numbers: Vec<u32> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut numbers_in_line = Vec::with_capacity(line.len());

        for char in line.chars() {
            if !char.is_numeric() {
                continue;
            }

            numbers_in_line.push(char);
        }

        let number = if numbers_in_line.len() == 1 {
            numbers_in_line[0].to_string().repeat(2)
        } else {
            format!(
                "{}{}",
                numbers_in_line.first().unwrap(),
                numbers_in_line.last().unwrap()
            )
        };

        let number: u32 = number.parse()?;
        numbers.push(number);
    }

    let sum: u32 = numbers.iter().sum();
    println!("The sum is: {sum}");

    Ok(())
}

#[allow(dead_code)]
fn read_input() -> color_eyre::Result<String> {
    let path = "improved-calibration.txt";
    let input = fs::read_to_string(path).wrap_err(format!("reading {path}"))?;

    Ok(input)
}
