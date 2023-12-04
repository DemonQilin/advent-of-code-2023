use aoc_recover_calibration_value::get_calibration_value;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let lines: Vec<&str> = include_str!("improved-calibration.txt").lines().collect();
    let calibrarion_value = get_calibration_value(&lines);

    println!("Calibration value is: {calibrarion_value}");

    Ok(())
}
