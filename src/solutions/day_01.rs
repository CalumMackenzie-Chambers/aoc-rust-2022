use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part_a(&self, input: &str) -> String {
        let mut max = 0;
        let mut current = 0;

        for line in input.lines() {
            if line.is_empty() {
                match current > max {
                    true => max = current,
                    false => (),
                }
                current = 0;
                continue;
            }
            current += line.parse::<i32>().unwrap();
        }

        match current > max {
            true => current.to_string(),
            false => max.to_string(),
        }
    }

    fn part_b(&self, input: &str) -> String {
        let mut maximum = [0; 3];
        let mut current = 0;

        for line in input.lines() {
            if line.is_empty() {
                insert_into_array(&mut maximum, current);
                current = 0;
                continue;
            }
            current += line.parse::<i32>().unwrap();
        }
        insert_into_array(&mut maximum, current);
        maximum.iter().sum::<i32>().to_string()
    }
}

fn insert_into_array(array: &mut [i32], value: i32) {
    if value < array[array.len() - 1] {
        return;
    }
    let index = array.iter().position(|&x| x < value).unwrap();
    for i in (index..array.len() - 1).rev() {
        array[i + 1] = array[i];
    }
    array[index] = value;
}

// -------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load;

    #[test]
    fn test_part_a() {
        let input = load(1, Some(true));
        let result = Day1::part_a(&Day1, &input);

        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part_b() {
        let input = load(1, Some(true));
        let result = Day1::part_b(&Day1, &input);

        assert_eq!(result, "45000");
    }

    #[test]
    fn insert_small_value_into_array() {
        let mut input = [7, 6, 5, 3];
        insert_into_array(&mut input, 1);

        assert_eq!(input, [7, 6, 5, 3])
    }

    #[test]
    fn insert_into_array_eq() {
        let mut input = [7, 6, 5, 3];
        insert_into_array(&mut input, 7);

        assert_eq!(input, [7, 7, 6, 5])
    }
}
