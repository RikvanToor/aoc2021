use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::IResult;
use pathfinding::directed::astar::astar;

use crate::days::Day;

pub struct Day15;

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
  let (cont, digits_str) = digit1(input)?;
  let res = digits_str
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();
  Ok((cont, res))
}

impl Day for Day15 {
  type Input = Vec<Vec<u32>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list1(newline, parse_line)(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let height = input.len();
    let width = input[0].len();

    let res = astar(
      &(0, 0),
      |(x, y)| {
        let mut res: Vec<((usize, usize), u32)> = vec![];
        if *x > 0 {
          res.push(((*x - 1, *y), input[*y][*x - 1]));
        }
        if *x < width - 1 {
          res.push(((*x + 1, *y), input[*y][*x + 1]));
        }
        if *y > 0 {
          res.push(((*x, *y - 1), input[*y - 1][*x]));
        }
        if *y < height - 1 {
          res.push(((*x, *y + 1), input[*y + 1][*x]));
        }
        res
      },
      |(x, y)| (width - x - 1 + height - y - 1) as u32,
      |(x, y)| *x == width - 1 && *y == height - 1,
    );

    res.unwrap().1
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let height = input.len();
    let width = input[0].len();

    let res = astar(
      &(0, 0),
      |(x, y)| {
        let mut poss: Vec<(usize, usize)> = vec![];
        if *x > 0 {
          poss.push((*x - 1, *y));
        }
        if *x < width * 5 - 1 {
          poss.push((*x + 1, *y));
        }
        if *y > 0 {
          poss.push((*x, *y - 1));
        }
        if *y < height * 5 - 1 {
          poss.push((*x, *y + 1));
        }
        let res: Vec<((usize, usize), u32)> = poss
          .iter()
          .map(|p| {
            let (x, y) = p;
            let cost = input[y % height][x % width] - 1 + (x / width) as u32 + (y / height) as u32;
            (*p, cost % 9 + 1)
          })
          .collect();
        res
      },
      |(x, y)| (width * 5 - x - 1 + height * 5 - y - 1) as u32,
      |(x, y)| *x == width * 5 - 1 && *y == height * 5 - 1,
    );

    res.unwrap().1
  }
}
