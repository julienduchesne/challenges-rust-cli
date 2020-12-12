use std::collections::HashMap;

use anyhow::Result;
use ndarray::{Array, Array2, Axis};

use super::super::challenge_config::ChallengeConfig;

pub struct Day3 {}

impl Day3 {
    fn solve_path(&self, array: Array2<usize>, path: Vec<isize>) -> usize {
        let mut trees_hit = 0;
        let mut current_row = 0;
        let mut current_col = 0;
        let row_count = array.len_of(Axis(0)) as isize;
        let col_count = array.len_of(Axis(1)) as isize;
        while current_row < row_count - 1 {
            current_row -= path[0];
            current_col = (current_col + path[1]) % col_count; // Possible to overflow
            if array.get((current_row as usize, current_col as usize)) == Some(&1) {
                trees_hit += 1;
            }
        }

        return trees_hit;
    }
}

impl ChallengeConfig for Day3 {
    fn title(&self) -> &str {
        return "Day 3: Toboggan Trajectory";
    }

    fn description(&self) -> &str {
        return "";
    }

    fn variables(&self) -> Vec<String> {
        return vec!["map".to_owned(), "paths".to_owned()];
    }

    fn solve(&self, variables: HashMap<&str, &str>) -> Result<String> {
        let lines: Vec<&str> = variables["map"]
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        let ncols = lines[0].len();
        let nrows = lines.len();
        let mut array: Array2<usize> = Array2::zeros((nrows, ncols));
        for (i, mut row) in array.axis_iter_mut(Axis(0)).enumerate() {
            // Perform calculations and assign to `row`; this is a trivial example:
            row.assign(&Array::from(
                lines[i]
                    .chars()
                    .map(|x| (x == '#') as usize)
                    .collect::<Vec<usize>>(),
            ));
        }
        let path_vecs: Vec<Vec<isize>> = variables["paths"]
            .split(";")
            .map(|x| x.split(",").map(|x| x.parse::<isize>().unwrap()).collect())
            .collect();

        let mut total = 1;
        let mut result = "".to_string();
        for (i, path_vec) in path_vecs.iter().enumerate() {
            let line_result = self.solve_path(array.clone(), path_vec.clone());
            total = total * line_result;
            result.push_str(format!("Path {}: {}\n", i, line_result).as_str());
        }
        result.push_str(format!("Total: {}", total).as_str());

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use rstest::rstest;

    use super::*;

    #[rstest(
        map,
        path,
        expected,
        // Single path (Part 1)
        case(
            "..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#",
            "-1,3",
            "Path 0: 7\nTotal: 7"
        ),
        // Multiple paths (Part 2)
        case(
            "..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#",
            "-1,1;-1,3;-1,5;-1,7;-2,1",
            "Path 0: 2\nPath 1: 7\nPath 2: 3\nPath 3: 4\nPath 4: 2\nTotal: 336"
        )
    )]
    fn solve(map: &str, path: &str, expected: &str) {
        let day = Day3 {};
        assert_eq!(
            day.solve(hashmap! {"map" => map, "paths" => path}).unwrap(),
            expected
        );
    }
}
