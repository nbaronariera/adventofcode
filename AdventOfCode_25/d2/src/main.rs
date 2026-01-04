use std::{
    cmp::{max, min},
    fs::File,
    io::{BufReader, Read},
};

type Range = (u64, u64);

struct OrderedRangeVector {
    ranges: Vec<Range>,
}

struct OrderedRangeVectorIterator<'a> {
    vector: &'a OrderedRangeVector,
    index: usize,
    current_value: u64,
}

impl OrderedRangeVector {
    fn new() -> OrderedRangeVector {
        OrderedRangeVector { ranges: Vec::new() }
    }

    fn iter(&self) -> OrderedRangeVectorIterator<'_> {
        let initial_value = self.ranges.first().map(|r| r.0).unwrap_or(0);

        OrderedRangeVectorIterator {
            vector: &self,
            index: 0,
            current_value: initial_value,
        }
    }

    fn add(&mut self, range: Range) {
        if self.ranges.first().is_none() || self.ranges.first().unwrap().0 > range.1 {
            self.ranges.insert(0, range);
        } else if self.ranges.last().unwrap().1 < range.0 {
            self.ranges.push(range);
        } else {
            let start_index = self.ranges.partition_point(|r| r.1 < range.0);

            if range.1 < self.ranges[start_index].0 {
                self.ranges.insert(start_index, range);
                return;
            }

            self.ranges[start_index].0 = min(self.ranges[start_index].0, range.0);
            self.ranges[start_index].1 = max(self.ranges[start_index].1, range.1);

            let mut next = start_index + 1;

            while next < self.ranges.len() && self.ranges[next].0 <= self.ranges[start_index].1 {
                self.ranges[start_index].1 = max(self.ranges[start_index].1, self.ranges[next].1);
                next += 1;
            }

            if next > start_index + 1 {
                self.ranges.drain((start_index + 1)..next);
            }
        }
    }
}

impl<'a> Iterator for OrderedRangeVectorIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.ranges.len() {
            return None;
        }

        let current_range = self.vector.ranges[self.index];
        let value_to_return = self.current_value;

        if self.current_value < current_range.1 {
            self.current_value += 1;
        } else {
            self.index += 1;
            if self.index < self.vector.ranges.len() {
                self.current_value = self.vector.ranges[self.index].0;
            }
        }

        Some(value_to_return)
    }
}

fn main() {
    p1();
    p2();
}

fn create_set() -> OrderedRangeVector {
    let f = File::open("./input.txt").expect("Unable to open file");
    let mut f = BufReader::new(f);

    let mut input: String = String::new();

    let mut ordered_vector = OrderedRangeVector::new();
    f.read_to_string(&mut input).expect("Expected content");

    input
        .trim()
        .split(",")
        .filter_map(|r| r.split_once("-"))
        .for_each(|(i, e)| {
            let start = i.parse::<u64>().expect("Expected value");
            let end = e.parse::<u64>().expect("Expected value");

            ordered_vector.add((start, end));
        });
    return ordered_vector;
}

fn p2() {
    let mut result = 0;
    let ordered_vector = create_set();

    for value in ordered_vector.iter() {
        let string = value.to_string();

        let mid = string.len() / 2;
        let mut size = mid;

        while size >= 1 {
            if string.len() % size != 0 {
                continue;
            }
            if string
                .as_bytes()
                .chunks(size)
                .all(|x| *x == string.as_bytes()[..size])
            {
                result += value;
                break;
            }
            size -= 1
        }
    }

    println!("El resultado es {result}");
}

fn p1() {
    let mut result = 0;
    let ordered_vector = create_set();

    for value in ordered_vector.iter() {
        let string = value.to_string();

        if string.len() % 2 != 0 {
            continue;
        }

        let mid = string.len() / 2;

        if string[..mid] == string[mid..] {
            result += value
        }
    }

    println!("El resultado es {result}");
}
