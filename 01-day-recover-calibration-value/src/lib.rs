mod utils;

use std::collections::HashMap;
use utils::{find_apperance, replace_in_index};

fn build_line_without_spellings(line: &str, spelling_map: &HashMap<String, String>) -> String {
    let mut new_line = String::from(line);
    let spelled_keys = spelling_map.keys().map(|k| &k[..]).collect::<Vec<_>>();

    if let Some((spelled, index)) = find_apperance(&new_line, &spelled_keys, true) {
        new_line = replace_in_index(&new_line, index, spelling_map.get(spelled).unwrap());
    }

    if let Some((spelled, index)) = find_apperance(&new_line, &spelled_keys, false) {
        new_line = replace_in_index(&new_line, index, spelling_map.get(spelled).unwrap());
    }

    new_line
}

pub fn get_calibration_value(records: &[&str]) -> u32 {
    let spelling_map = HashMap::from([
        ("one".to_string(), "1".to_string()),
        ("two".to_string(), "2".to_string()),
        ("three".to_string(), "3".to_string()),
        ("four".to_string(), "4".to_string()),
        ("five".to_string(), "5".to_string()),
        ("six".to_string(), "6".to_string()),
        ("seven".to_string(), "7".to_string()),
        ("eight".to_string(), "8".to_string()),
        ("nine".to_string(), "9".to_string()),
    ]);

    records
        .iter()
        .map(|&line| build_line_without_spellings(line, &spelling_map))
        .map(|line| line.split(|c: char| !c.is_numeric()).collect::<String>())
        .map(|numbers| numbers[0..1].to_string() + &numbers[numbers.len() - 1..])
        .map(|number| number.parse::<u32>().unwrap())
        .sum()
}
