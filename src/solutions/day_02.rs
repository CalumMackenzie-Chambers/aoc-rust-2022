use crate::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn part_a(&self, input: &str) -> String {
        input.lines().map(line_result).sum::<u32>().to_string()
    }

    fn part_b(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| calculate_move_and_points(line).unwrap())
            .sum::<u32>()
            .to_string()
    }
}

fn map_to_number(letter: char, part: char) -> u8 {
    if part == 'a' {
        match letter {
            'A' | 'X' => return 1,
            'B' | 'Y' => return 2,
            'C' | 'Z' => return 3,
            _ => return 4,
        }
    }
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    }
}

fn parse_line(line: &str, part: char) -> [u8; 2] {
    let mut result = [0; 2];
    let mut index = 0;
    for c in line.chars() {
        if c == ' ' {
            index += 1;
            continue;
        }
        result[index] = map_to_number(c, part);
    }
    result
}

fn wrap_add(a: u8, b: u8, max: u8) -> Result<u8, &'static str> {
    if a > max || b > max {
        return Err("inputs cannot be greater than the maximum value");
    }
    let sum = a + b;
    if sum > max {
        return Ok(sum - max);
    }
    Ok(sum)
}

fn line_result(line: &str) -> u32 {
    let parsed = parse_line(line, 'a');
    let mut result = parsed[1] as u32;

    if parsed[0] == parsed[1] {
        return (result + 3) as u32;
    }

    let mut wrapped = [
        wrap_add(parsed[0], 1, 3).unwrap(),
        wrap_add(parsed[1], 1, 3).unwrap(),
    ];
    if wrapped[0].abs_diff(wrapped[1]) == 2 {
        wrapped = [
            wrap_add(parsed[0], 2, 3).unwrap(),
            wrap_add(parsed[1], 2, 3).unwrap(),
        ];
    }

    match wrapped[1] > wrapped[0] {
        true => result += 6,
        false => (),
    }

    result
}

fn calculate_move_and_points(line: &str) -> Result<u32, &'static str> {
    let parsed = parse_line(line, 'b');
    let result = parsed[1] as u32;

    match parsed[1] {
        6 => Ok(result + wrap_add(parsed[0], 1, 3).unwrap() as u32),
        3 => Ok(result + parsed[0] as u32),
        0 => Ok(result + wrap_add(parsed[0], 2, 3).unwrap() as u32),
        _ => Err("Parsing error"),
    }
}

// -------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    #[test]
    fn test_part_a() {
        let input = load(2, Some(true));
        let result = Day2::part_a(&Day2, &input);

        assert_eq!(result, "15");
    }

    #[test]
    fn test_part_b() {
        let input = load(2, Some(true));
        let result = Day2::part_b(&Day2, &input);

        assert_eq!(result, "12")
    }
}
