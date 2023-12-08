mod models;

use models::AlmanacMap;

pub fn get_lowest_location_of_seeds_first_part(input: &str) -> usize {
    let input_lines = input.lines().collect::<Vec<_>>();

    let mut groups = input_lines
        .split(|line| line.is_empty())
        .collect::<Vec<_>>();

    let seeds = groups.remove(0)[0]
        .split_ascii_whitespace()
        .filter_map(|maybe_seed| maybe_seed.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let maps = groups
        .into_iter()
        .map(|map| {
            let ranges = map[1..]
                .iter()
                .map(|range| {
                    range
                        .split_ascii_whitespace()
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            AlmanacMap::new(&ranges)
        })
        .collect::<Vec<_>>();

    let locations = seeds
        .into_iter()
        .map(move |seed| maps.iter().fold(seed, |seed, map| map.get(seed)))
        .collect::<Vec<_>>();

    locations.into_iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_35() {
        let input = include_str!("seeds-almanac-test-input.txt");
        let result = get_lowest_location_of_seeds_first_part(input);

        assert_eq!(result, 35);
    }
}
