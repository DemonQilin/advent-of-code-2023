struct ScratchCard<'a> {
    card_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
}

impl<'a> ScratchCard<'a> {
    fn get_total_matching_numbers(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn get_points(&self) -> u32 {
        let total_matches = self.get_total_matching_numbers();

        if total_matches > 0 {
            2u32.pow(total_matches as u32 - 1)
        } else {
            0
        }
    }
}

fn main() {
    let scratchcards = include_str!("scratchcards-input.txt")
        .lines()
        .map(|line| line.split(':').last().unwrap().trim())
        .map(|raw_numbers| {
            let mut numbers = raw_numbers.split('|').collect::<Vec<_>>();
            let card_numbers = numbers
                .pop()
                .unwrap_or_else(|| panic!("Numbers were expected after '|' in \"{raw_numbers}\""))
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            let winning_numbers = numbers
                .pop()
                .unwrap_or_else(|| panic!("Numbers were expected before '|' in \"{raw_numbers}\""))
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            ScratchCard {
                card_numbers,
                winning_numbers,
            }
        })
        .collect::<Vec<_>>();

    let total_points = scratchcards
        .iter()
        .map(|card| card.get_points())
        .sum::<u32>();

    println!("Total points are {total_points}");
}
