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

    fn get_matching_ranges(&self, other_range: &AlmanacMapRange) -> Vec<&AlmanacMapRange> {
        self.ranges
            .iter()
            .filter(|range| {
                let final_range = range.source_start + range.length;
                let final_other = other_range.destination_start + other_range.length;

                let is_range_out = range.source_start < other_range.destination_start
                    && final_range < other_range.destination_start
                    || range.source_start > final_other && final_range > final_other;

                !is_range_out
            })
            .collect()
    }

    pub fn get_direct_map(&self, other: &Self) -> Self {
        let direct_ranges = other
            .ranges
            .iter()
            .flat_map(|range| {
                let matching_ranges = self.get_matching_ranges(range);

                if matching_ranges.is_empty() {
                    let range = AlmanacMapRange::new(
                        range.destination_start,
                        range.source_start,
                        range.length,
                    );

                    return vec![range];
                };

                let matching_ranges_length = matching_ranges.len();
                let mut direct_ranges: Vec<AlmanacMapRange> =
                    Vec::with_capacity(matching_ranges_length);

                for (index, matching_range) in matching_ranges.into_iter().enumerate() {
                    let matching_range_end = matching_range.source_start + matching_range.length;
                    let original_range_end = range.destination_start + range.length;
                    let direct_range: AlmanacMapRange;

                    if range.destination_start <= matching_range.source_start
                        && original_range_end >= matching_range_end
                    {
                        let diff = matching_range.source_start - range.destination_start;

                        direct_range = AlmanacMapRange::new(
                            matching_range.destination_start,
                            range.source_start + diff,
                            matching_range.length,
                        );
                    } else if range.destination_start >= matching_range.source_start
                        && original_range_end <= matching_range_end
                    {
                        let diff = range.destination_start - matching_range.source_start;

                        direct_range = AlmanacMapRange::new(
                            matching_range.destination_start + diff,
                            range.source_start,
                            range.length,
                        );
                    } else if range.destination_start < matching_range.source_start
                        && original_range_end <= matching_range_end
                    {
                        let start_diff = matching_range.source_start - range.destination_start;

                        direct_range = AlmanacMapRange::new(
                            matching_range.destination_start,
                            range.source_start + start_diff,
                            range.length - start_diff,
                        );
                    } else {
                        let start_diff = range.destination_start - matching_range.source_start;

                        direct_range = AlmanacMapRange::new(
                            matching_range.destination_start + start_diff,
                            range.source_start,
                            matching_range.length - start_diff,
                        );
                    }

                    let (mut before_missing, mut next_missing) = (None, None);

                    if let Some(previous_range) = direct_ranges.last() {
                        let previous_range_end =
                            previous_range.source_start + previous_range.length;
                        let are_continuos = previous_range_end == direct_range.source_start;

                        if !are_continuos {
                            let diff = previous_range_end - range.source_start;

                            before_missing = Some(AlmanacMapRange::new(
                                range.destination_start + diff,
                                previous_range_end,
                                direct_range.source_start - previous_range_end,
                            ));
                        }
                    } else if direct_range.source_start > range.source_start {
                        before_missing = Some(AlmanacMapRange::new(
                            range.destination_start,
                            range.source_start,
                            direct_range.source_start - range.source_start,
                        ));
                    }

                    let direct_range_end = direct_range.source_start + direct_range.length;
                    let extends_to_original_end =
                        direct_range_end == range.source_start + range.length;

                    if index == matching_ranges_length - 1 && !extends_to_original_end {
                        let diff = direct_range_end - range.source_start;

                        next_missing = Some(AlmanacMapRange::new(
                            range.destination_start + diff,
                            direct_range_end,
                            range.source_start + range.length - direct_range_end,
                        ));
                    }

                    if let Some(before_missing) = before_missing {
                        direct_ranges.push(before_missing);
                    }

                    direct_ranges.push(direct_range);

                    if let Some(next_missing) = next_missing {
                        direct_ranges.push(next_missing);
                    }
                }

                direct_ranges
            })
            .collect::<Vec<_>>();

        Self {
            ranges: direct_ranges,
        }
    }

    pub fn get_lowest_destination_value(&self) -> usize {
        self.ranges
            .iter()
            .min_by_key(|range| range.destination_start)
            .unwrap()
            .destination_start
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
