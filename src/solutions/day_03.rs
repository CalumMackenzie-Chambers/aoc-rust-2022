use std::collections::HashSet;

use crate::Solution;

pub struct Day3;

impl Solution for Day3 {
    fn part_a(&self, input: &str) -> String {
        let mut res = 0;

        for line in input.lines() {
            let backpacks = split_string(line).unwrap();
            res += map_char(find_common_char(&backpacks).unwrap()).unwrap() as u32;
        }

        res.to_string()
    }

    fn part_b(&self, input: &str) -> String {
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|chunk| {
                map_char(find_common_char(&[chunk[0], chunk[1], chunk[2]]).unwrap()).unwrap() as u32
            })
            .sum::<u32>()
            .to_string()
    }
}

fn split_string(input: &str) -> Result<[&str; 2], &str> {
    let len = input.len();
    if len % 2 != 0 {
        return Err("Must give a string with even length");
    }

    let half = len / 2;
    Ok([&input[..half], &input[half..]])
}

fn find_common_char(input: &[&str]) -> Result<char, &'static str> {
    let mut set:HashSet<char> = HashSet::from_iter(input[0].chars());
    for elem in input.iter().skip(1) {
        set = set.intersection(&HashSet::from_iter(elem.chars())).cloned().collect();
    }
    
    match set.len() {
        0 => Err("No common char found"),
        1 => Ok(*set.iter().next().unwrap()),
        _ => Err("More than one common char found"),
    }
}

fn map_char(c: char) -> Result<u8, String> {
    match c {
        'a'..='z' => Ok(c as u8 - 96),
        'A'..='Z' => Ok(c as u8 - 38),
        _ => Err(format!("Invalid character: {}", c)),
    }
}

// -------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    #[test]
    fn test_part_a() {
        let input = load(3, Some(true));
        let result = Day3::part_a(&Day3, &input);

        assert_eq!(result, "157")
    }

    #[test]
    fn test_part_b() {
        let input = load(3, Some(true));
        let result = Day3::part_b(&Day3, &input);

        assert_eq!(result, "70")
    }
}
