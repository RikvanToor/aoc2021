use itertools::concat;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, newline};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashMap;

use crate::days::Day;

pub struct Day05;

type Line = ((i32, i32), (i32, i32));

fn parse_line(input: &str) -> IResult<&str, Line> {
  let (cont, (x1, _, y1, _, x2, _, y2)) =
    tuple((i32, tag(","), i32, tag(" -> "), i32, tag(","), i32))(input)?;
  Ok((cont, ((x1, y1), (x2, y2))))
}

impl Day for Day05 {
  type Input = Vec<Line>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_line)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let points: Vec<(i32, i32)> = concat(
      input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .map(|((x1, y1), (x2, y2))| {
          if y1 == y2 {
            (min(*x1, *x2)..=max(*x1, *x2))
              .map(|x| (x, *y1))
              .collect::<Vec<(i32, i32)>>()
          } else {
            (min(*y1, *y2)..=max(*y1, *y2))
              .map(|y| (*x1, y))
              .collect::<Vec<(i32, i32)>>()
          }
        }),
    );

    let mut points_count: HashMap<(i32, i32), i32> = HashMap::new();
    for p in points {
      *points_count.entry(p).or_default() += 1;
    }

    let answer = points_count.iter().filter(|(_, x)| x > &&1).count();

    answer
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let points: Vec<(i32, i32)> = concat(input.iter().cloned().map(|((x1, y1), (x2, y2))| {
      if y1 == y2 {
        (min(x1, x2)..=max(x1, x2))
          .map(|x| (x, y1))
          .collect::<Vec<(i32, i32)>>()
      } else if x1 == x2 {
        (min(y1, y2)..=max(y1, y2))
          .map(|y| (x1, y))
          .collect::<Vec<(i32, i32)>>()
      } else {
        let facx = if x2 > x1 { 1 } else { -1 };
        let facy = if y2 > y1 { 1 } else { -1 };
        (0..=(i32::abs(x2 - x1)))
          .map(|d| (x1 + d * facx, y1 + d * facy))
          .collect()
      }
    }));

    let mut points_count: HashMap<(i32, i32), i32> = HashMap::new();
    for p in points {
      *points_count.entry(p).or_default() += 1;
    }

    let answer = points_count.iter().filter(|(_, x)| x > &&1).count();

    answer
  }
}
