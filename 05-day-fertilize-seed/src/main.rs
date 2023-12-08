use aoc_fertilize_seed::{
    get_lowest_location_of_seeds_first_part, get_lowest_location_of_seeds_second_part,
};

fn main() {
    let lowest_location =
        get_lowest_location_of_seeds_first_part(include_str!("seeds-almanac-input.txt"));

    println!("lowest location of 1 start = {lowest_location}");

    let lowest_location =
        get_lowest_location_of_seeds_second_part(include_str!("seeds-almanac-input.txt"));

    println!("lowest location of 2 start = {lowest_location}");
}
