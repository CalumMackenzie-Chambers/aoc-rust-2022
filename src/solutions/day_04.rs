use std::cmp::Ordering;

use crate::Solution;

pub struct Day4;

impl Solution for Day4 {
    fn part_a(&self, input: &str) -> String {
        input
            .lines()
            .filter_map(|element| {
                let numbers = extract_numbers(element);
                match is_contained(&numbers) {
                    true => Some(numbers),
                    false => None,
                }
            })
            .count()
            .to_string()
    }

    fn part_b(&self, input: &str) -> String {
        input
            .lines()
            .filter_map(|element| {
                let numbers = extract_numbers(element);
                match does_overlap(&numbers) {
                    true => Some(numbers),
                    false => None,
                }
            })
            .count()
            .to_string()
    }
}

// function to extract numbers from a string containing any characters
fn extract_numbers(input: &str) -> Vec<i32> {
    input
        .split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_contained(input: &Vec<i32>) -> bool {
    if input.len() != 4 {
        return false;
    }

    match input[0].cmp(&input[2]) {
        Ordering::Less => input[3] <= input[1],
        Ordering::Equal => true,
        Ordering::Greater => input[1] <= input[3],
    }
}

fn does_overlap(input: &Vec<i32>) -> bool {
    if input.len() != 4 {
        return false;
    }

    match input[0].cmp(&input[2]) {
        Ordering::Less => input[1] >= input[2],
        Ordering::Equal => true,
        Ordering::Greater => input[3] >= input[0],
    }
}

// -------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    #[test]
    fn test_part_a() {
        let input = load(4, Some(true));
        let result = Day4::part_a(&Day4, &input);

        assert_eq!(result, "2")
    }

    #[test]
    fn test_part_b() {
        let input = load(4, Some(true));
        let result = Day4::part_b(&Day4, &input);

        assert_eq!(result, "4")
    }
}
