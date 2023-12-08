#[derive(Debug)]
struct ScratchCard<'a> {
    id: usize,
    card_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
    quantity: u32,
}

impl<'a> ScratchCard<'a> {
    fn new<'b: 'a>(
        id: usize,
        card_numbers: Vec<&'b str>,
        winning_numbers: Vec<&'b str>,
    ) -> ScratchCard<'a> {
        ScratchCard {
            id,
            card_numbers,
            winning_numbers,
            quantity: 1,
        }
    }

    fn get_total_matching_numbers(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    #[allow(dead_code)]
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
    let mut scratchcards = include_str!("scratchcards-input.txt")
        .lines()
        .map(|line| {
            let mut card_segments = line.split([':', '|']).collect::<Vec<_>>();
            let card_numbers = card_segments
                .pop()
                .unwrap_or_else(|| panic!("Numbers were expected after '|' in \"{line}\""))
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            let winning_numbers = card_segments
                .pop()
                .unwrap_or_else(|| panic!("Numbers were expected before '|' in \"{line}\""))
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();

            let id = card_segments
                .pop()
                .unwrap_or_else(|| panic!("Description was expected after \":\" in \"{line}\""))
                .split(|char: char| !char.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("The card was expected to have a number"));

            ScratchCard::new(id, card_numbers, winning_numbers)
        })
        .collect::<Vec<_>>();

    for index in 0..scratchcards.len() {
        let scratchcard = &scratchcards[index];
        let scratchcard_id = scratchcard.id;
        let total_matches = scratchcard.get_total_matching_numbers();
        let increase = scratchcard.quantity;

        for j in 0..total_matches {
            let scratch_copy = &mut scratchcards[scratchcard_id + j];
            scratch_copy.quantity += increase;
        }
    }

    let total_points = scratchcards
        .iter()
        .map(|scratchcard| scratchcard.quantity)
        .sum::<u32>();

    println!("The total cards is {total_points}");
}
