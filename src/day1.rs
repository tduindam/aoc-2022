use std::fs::File;
use std::io::{BufReader, Lines};

use crate::reader::read_lines;
use crate::Error::InputError;
use crate::Result;

pub fn part_1_and_two() -> Result<(u32, u32)> {
    let input = read_lines("input/day1-1");
    if let Ok(lines) = input {
        let mut calories = count_calories(lines);
        let (index, max) = find_elf_with_most_calories(&calories);
        let top_3_sum = find_top_three(&mut calories);
        println!("Part 1: Elf {index} has most calories ({max})");
        println!("Part 2: Top 3 elves have {top_3_sum} calories");
        Ok((max, top_3_sum))
    } else {
        Err(InputError)
    }
}

fn count_calories(lines: Lines<BufReader<File>>) -> Vec<u32> {
    let mut cur_group = Vec::<u32>::new();
    let mut groups = Vec::<Vec<u32>>::new();

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() && cur_group.len() > 0 {
                groups.push(cur_group.clone());
                cur_group.clear()
            } else {
                cur_group.push(line.parse::<u32>().unwrap())
            }
        }
    }
    if cur_group.len() > 0 {
        groups.push(cur_group);
    }
    groups.iter().map(|group| group.iter().sum()).collect()
}

fn find_top_three(calories: &mut Vec<u32>) -> u32 {
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(3).sum()
}

fn find_elf_with_most_calories(calories: &Vec<u32>) -> (usize, u32) {
    let max = calories.iter().enumerate().max_by_key(|(_, val)| *val);
    let (index, max) = max.unwrap();
    (index, *max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_small() {
        let expected_groups = [6000, 4000, 11000, 24000, 10000];
        let calories = count_calories(read_lines("input/day1-1-small").unwrap());
        assert_eq!(calories, expected_groups);

        let (index, max) = find_elf_with_most_calories(&calories);
        assert_eq!(index, 3);
        assert_eq!(max, 24000);
    }

    #[test]
    fn part_one_and_two() {
        let (part_1, part_2) = part_1_and_two().unwrap();
        assert_eq!(part_1, 72511);
        assert_eq!(part_2, 212117);
    }
}
