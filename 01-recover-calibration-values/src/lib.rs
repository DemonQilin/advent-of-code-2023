pub fn get_calibration_value(records: &[&str]) -> u32 {
    records
        .iter()
        .map(|line| line.split(|c: char| !c.is_numeric()).collect::<String>())
        .map(|numbers| numbers[0..1].to_string() + &numbers[numbers.len() - 1..])
        .map(|number| number.parse::<u32>().unwrap())
        .sum()
}
