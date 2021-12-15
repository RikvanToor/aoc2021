use crate::parser::digit_grid;
use nom::IResult;
use std::collections::HashSet;

use crate::days::Day;

pub struct Day09;

fn find_low_points(input: &[Vec<u32>]) -> Vec<(usize, usize)> {
  let height = input.len();
  let width = input[0].len();
  let mut res = vec![];
  for y in 0..height {
    for x in 0..width {
      let val = input[y][x];
      if (x == 0 || val < input[y][x - 1])
        && (x == width - 1 || val < input[y][x + 1])
        && (y == 0 || val < input[y - 1][x])
        && (y == height - 1 || val < input[y + 1][x])
      {
        res.push((x, y));
      }
    }
  }
  res
}

fn expand_basin(
  basin_points: &mut HashSet<(usize, usize)>,
  input: &[Vec<u32>],
  point: (usize, usize),
) {
  let (x, y) = point;
  let height = input.len();
  let width = input[0].len();
  let adjacents = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
  for p in adjacents {
    let (x2, y2) = p;
    if x2 < width && y2 < height && input[y2][x2] < 9 && !basin_points.contains(&p) {
      basin_points.insert(p);
      expand_basin(basin_points, input, p);
    }
  }
}

impl Day for Day09 {
  type Input = Vec<Vec<u32>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    digit_grid(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let low_points = find_low_points(input);
    low_points
      .iter()
      .map(|(x, y)| input[*y][*x] + 1)
      .sum::<u32>()
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let low_points = find_low_points(input);

    let mut basin_sizes: Vec<u32> = low_points
      .iter()
      .map(|p| {
        let mut basin_points = HashSet::from([*p]);
        expand_basin(&mut basin_points, input, *p);
        basin_points.len() as u32
      })
      .collect();

    basin_sizes.sort_by(|a, b| b.cmp(a));

    basin_sizes[0..=2].iter().product::<u32>()
  }
}
