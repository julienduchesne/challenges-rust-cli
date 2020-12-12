use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

use super::super::challenge_config::ChallengeConfig;

pub struct Day6 {}

impl Day6 {}

impl ChallengeConfig for Day6 {
    fn title(&self) -> &str {
        return "Day 6: Custom Customs";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["answers".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let input_without_spaces = variables["answers"].replace(" ", "");
        let groups: Vec<&str> = input_without_spaces
            .split("\n\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();

        // Part 1: count number of distinct letters in each group
        // Part 2: count number of distinct letters that are in each line in each group
        let mut part_one = 0;
        let mut part_two = 0;
        for group in groups {
            let people_count = group.matches('\n').count() + 1;
            let distinct_letters = group.chars().filter(|x| x.is_alphabetic()).unique();
            for letter in distinct_letters {
                part_one += 1;
                if group.matches(letter).count() == people_count {
                    part_two += 1;
                }
            }
        }

        return Ok(format!("Part 1: {}\nPart 2: {}", part_one, part_two).to_string());
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        answers,
        expected,
        case(
            "abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b",
            "Part 1: 11\nPart 2: 6"
        )
    )]
    fn solve(answers: &str, expected: &str) {
        let day = Day6 {};
        assert_eq!(
            day.solve(hashmap! {"answers" => answers}).unwrap(),
            expected
        );
    }
}
