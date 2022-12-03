use std::collections::HashSet;

use crate::Error::ValidationError;
use crate::Result;

fn value(input: char) -> Option<u32> {
    let ascii = input as u32;
    let score = match ascii {
        s if s >= 65 && s <= 90 => Some(s - 64 + 26),
        s if s >= 97 && s <= 122 => Some(s - 96),
        _ => None,
    };
    score
}

fn process_line_part_one(line: &str) -> Result<u32> {
    if line.len() % 2 != 0 {
        return Err(ValidationError("Line must have even amount of elements"));
    }
    let comp_size = line.len() / 2;
    let comp_1 = &line[..comp_size];
    let comp_2 = &line[comp_size..];
    let map_1: HashSet<char> = HashSet::from_iter(comp_1.chars());
    let doubles: Vec<u32> = comp_2
        .chars()
        .filter(|c| map_1.contains(c))
        .filter_map(|c| value(c))
        .collect();
    Ok(doubles[0])
}

#[cfg(test)]
mod tests {
    use crate::reader::read_lines_filter_ok;
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn part_one_small() {
        let lines = read_lines_filter_ok("input/day3-1-small");
        let priorities: Vec<u32> = lines
            .iter()
            .filter_map(|l| process_line_part_one(&l).ok())
            .collect();
        assert_eq!(priorities, [16, 38, 42, 22, 20, 19]);
        assert_eq!(priorities.iter().sum::<u32>(), 157);
    }
    #[test]
    fn part_one() {
        let lines = read_lines_filter_ok("input/day3-1");
        let priorities: Vec<u32> = lines
            .iter()
            .filter_map(|l| process_line_part_one(&l).ok())
            .collect();
        assert_eq!(priorities.iter().sum::<u32>(), 8202);
    }
    #[test]
    fn part_two_small() {
        let priorities = process_part_two("input/day3-2-small");
        assert_eq!(priorities, [18, 52])
    }
    #[test]
    fn part_two() {
        let priorities = process_part_two("input/day3-1");
        assert_eq!(priorities.iter().sum::<u32>(), 2864);
    }

    fn process_part_two(filename: &str) -> Vec<u32> {
        let lines = read_lines_filter_ok(filename);
        let mut badges = Vec::<char>::new();
        for i in (0..lines.len()).step_by(3) {
            let line_1 = &lines[i];
            let line_2 = &lines[i + 1];
            let line_3 = &lines[i + 2];
            let mut map: HashMap<char, u32> = HashMap::new();
            count_chars(line_1.chars().collect(), &mut map);
            count_chars(line_2.chars().collect(), &mut map);
            count_chars(line_3.chars().collect(), &mut map);
            let badge: Vec<(char, u32)> = map
                .iter()
                .filter(|(_, v)| **v == 3)
                .map(|(k, v)| (*k, *v))
                .collect();
            badges.push(badge[0].0);
        }
        let priorities: Vec<u32> = badges.iter().filter_map(|c| value(*c)).collect();
        priorities
    }

    fn count_chars(mut line_1: Vec<char>, map: &mut HashMap<char, u32>) {
        line_1.sort();
        line_1.dedup();

        for c in line_1 {
            map.entry(c).and_modify(|f| *f += 1).or_insert(1);
        }
    }
}
