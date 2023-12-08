use aoc_fertilize_seed::get_lowest_location_of_seeds_first_part;

fn main() {
    let lowest_location =
        get_lowest_location_of_seeds_first_part(include_str!("seeds-almanac-input.txt"));

    println!("lowest location = {lowest_location}");
}
