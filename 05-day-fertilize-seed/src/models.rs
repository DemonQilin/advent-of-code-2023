#[derive(Debug)]
pub struct AlmanacMap {
    ranges: Vec<AlmanacMapRange>,
}

impl AlmanacMap {
    pub fn new(ranges: &[Vec<usize>]) -> Self {
        let mut ranges = ranges
            .iter()
            .map(|range| AlmanacMapRange::new(range[0], range[1], range[2]))
            .collect::<Vec<_>>();

        ranges.sort_by(|range_a, range_b| range_a.source_start.cmp(&range_b.source_start));

        Self { ranges }
    }

    pub fn get(&self, source_number: usize) -> usize {
        let destination_value = self
            .ranges
            .iter()
            .find(|range| range.can_map_value(source_number))
            .and_then(|range| range.get_value(source_number));

        if let Some(value) = destination_value {
            value
        } else {
            source_number
        }
    }
}

#[derive(Debug)]
struct AlmanacMapRange {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl AlmanacMapRange {
    fn new(destination_start: usize, source_start: usize, length: usize) -> Self {
        Self {
            destination_start,
            source_start,
            length,
        }
    }

    fn can_map_value(&self, source_value: usize) -> bool {
        source_value >= self.source_start && source_value < self.source_start + self.length
    }

    fn get_value(&self, source_value: usize) -> Option<usize> {
        if self.can_map_value(source_value) {
            let distance = source_value - self.source_start;

            Some(self.destination_start + distance)
        } else {
            None
        }
    }
}
