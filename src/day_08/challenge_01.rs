/*
--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390
Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
The top-middle 5 is visible from the top and right.
The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
The left-middle 5 is visible, but only from the right.
The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?
*/

use std::cmp;

const INPUT_FIXTURE_PATH: &str = "./src/day_08/fixtures/input.txt";

#[derive(Debug)]
struct TreeCoordinates {
    x: usize,
    y: usize
}

pub struct Forest {
    trees: Vec<Vec<i32>>
}

impl Forest {
    pub fn new() -> Self {
        Forest {
            trees: vec![]
        }
    }

    pub fn add_row_of_trees(&mut self, row_of_trees: Vec<i32>) {
        self.trees.push(row_of_trees);
    }

    pub fn visible_tree_count(&self) -> usize {
        self.trees.iter().enumerate().map(|(x, row_of_trees)| {
            let visible_row_tree_count = row_of_trees.iter().enumerate()
                .filter(|(y, tree)| self.is_tree_visible(&TreeCoordinates { x, y: *y }, tree))
                .count();

            visible_row_tree_count
        }).sum()
    }

    pub fn highest_scenic_score(&self) -> i32 {
        self.trees.iter().enumerate().map(|(x, row)| {
            row.iter().enumerate().map(|(y, tree_height)| {
                let coords = TreeCoordinates { x, y };

                self.tree_scenic_score(&coords, tree_height)
            }).max().unwrap_or(0)
        }).max().unwrap_or(0)
    }

    fn tree_scenic_score(&self, coords: &TreeCoordinates, tree_height: &i32) -> i32 {
        let [
            mut row_slice_1,
            row_slice_2,
            mut col_slice_1,
            col_slice_2
        ] = self.coordinate_slices(coords);

        row_slice_1.reverse();
        col_slice_1.reverse();

        let l_score = self.position_to_score(row_slice_1.iter().enumerate().position(|(index, &v)| v >= *tree_height || index + 1 == row_slice_1.len()));
        let r_score = self.position_to_score(row_slice_2.iter().enumerate().position(|(index, &v)| v >= *tree_height || index + 1 == row_slice_2.len()));
        let u_score = self.position_to_score(col_slice_1.iter().enumerate().position(|(index, &v)| v >= *tree_height || index + 1 == col_slice_1.len()));
        let d_score = self.position_to_score(col_slice_2.iter().enumerate().position(|(index, &v)| v >= *tree_height || index + 1 == col_slice_2.len()));

        l_score * r_score * u_score * d_score
    }

    fn position_to_score(&self, position: Option<usize>) -> i32 {
        match position {
            None => 0,
            Some(pos) => pos.to_string().parse::<i32>().unwrap_or(-1) + 1
        }
    }

    fn coordinate_slices(&self, coords: &TreeCoordinates) -> [Vec<i32>; 4] {
        let &rows_count = &self.trees.len();
        let &cols_count = &self.trees[coords.x].len();
        let row_pos = cmp::min(coords.x, rows_count - 1);
        let col_pos = cmp::min(coords.y, cols_count - 1);

        let col_values = &self.trees.iter()
            .map(|row| *row.get(coords.y).unwrap_or(&-1))
            .collect::<Vec<i32>>();

        let row_slice_1 = self.trees[coords.x][0..col_pos].to_vec();
        let row_slice_2 = self.trees[coords.x][(col_pos + 1)..cols_count].to_vec();

        let col_slice_1 = col_values[0..row_pos].to_vec();
        let col_slice_2 = col_values[(row_pos + 1)..rows_count].to_vec();

        [row_slice_1, row_slice_2, col_slice_1, col_slice_2]
    }

    fn is_tree_visible(&self, coords: &TreeCoordinates, tree_height: &i32) -> bool {
        let [
            row_slice_1,
            row_slice_2,
            col_slice_1,
            col_slice_2
        ] = self.coordinate_slices(coords);

        row_slice_1.is_empty()
            || row_slice_2.is_empty()
            || col_slice_1.is_empty()
            || col_slice_2.is_empty()
            || row_slice_1.iter().max().unwrap_or(&-1) < tree_height
            || row_slice_2.iter().max().unwrap_or(&-1) < tree_height
            || col_slice_1.iter().max().unwrap_or(&-1) < tree_height
            || col_slice_2.iter().max().unwrap_or(&-1) < tree_height
    }
}

pub fn solve() {
    let result = solve_challenge(INPUT_FIXTURE_PATH);

    assert_eq!("1835", result);

    adventofcode2022::print_results(&result, "08", "01")
}

fn solve_challenge(fixture_path: &str) -> String {
    let forest = load_forest_map(fixture_path);

    forest.visible_tree_count().to_string()
}

pub fn load_forest_map(fixture_path: &str) -> Forest {
    let input = adventofcode2022::read_file_to_lines(fixture_path);
    let mut forest = Forest::new();

    for line in input {
        forest.add_row_of_trees(
            line.chars().into_iter()
                .map(|c| c.to_string().parse::<i32>().unwrap_or(-1))
                .collect::<Vec<i32>>()
        );
    }

    forest
}
