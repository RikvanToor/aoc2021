use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32};
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;
use nom::IResult;

use crate::days::Day;

pub struct Day13;

fn parse_dot(input: &str) -> IResult<&str, (u32, u32)> {
  let (cont, (x, _, y)) = tuple((u32, tag(","), u32))(input)?;
  Ok((cont, (x, y)))
}

fn parse_fold(input: &str) -> IResult<&str, FoldAlong> {
  let (cont, _) = tag("fold along ")(input)?;
  let (cont, ax) = alt((tag("x"), tag("y")))(cont)?;
  let (cont, (_, i)) = tuple((tag("="), u32))(cont)?;
  let res = match ax {
    "x" => FoldAlong::X(i),
    "y" => FoldAlong::Y(i),
    _ => panic!("Unknown axis {}", ax),
  };
  Ok((cont, res))
}

#[derive(Debug, Clone, Copy)]
pub enum FoldAlong {
  X(u32),
  Y(u32),
}

fn run(points: &[(u32, u32)], folds: &[FoldAlong]) -> Vec<(u32, u32)> {
  let mut dots: Vec<(u32, u32)> = points.to_vec();
  for f in folds {
    for (x, y) in &mut dots {
      match f {
        FoldAlong::X(i) => {
          if *x >= *i {
            *x = *i - (*x - *i);
          }
        }
        FoldAlong::Y(i) => {
          if *y >= *i {
            *y = *i - (*y - *i);
          }
        }
      }
    }
    dots.sort_unstable();
    dots.dedup();
    match f {
      FoldAlong::X(i) => {
        dots.retain(|(x, _)| x != i);
      }
      FoldAlong::Y(i) => {
        dots.retain(|(_, y)| y != i);
      }
    }
  }
  dots
}

impl Day for Day13 {
  type Input = (Vec<(u32, u32)>, Vec<FoldAlong>);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, points) = separated_list0(newline, parse_dot)(input)?;
    let (cont, (_, folds)) = tuple((many1(newline), separated_list0(newline, parse_fold)))(cont)?;
    Ok((cont, (points, folds)))
  }

  type Output1 = usize;

  fn part_1((points, folds): &Self::Input) -> Self::Output1 {
    let dots = run(points, &[folds[0]]);

    dots.len()
  }

  type Output2 = String;

  fn part_2((points, folds): &Self::Input) -> Self::Output2 {
    let dots = run(points, folds);
    let xs = dots.iter().map(|(x, _)| x);
    let max_x = xs.clone().max().unwrap();
    let min_x = xs.min().unwrap();
    let ys = dots.iter().map(|(_, y)| y);
    let max_y = ys.clone().max().unwrap();
    let min_y = ys.min().unwrap();

    let mut res = String::from("");

    for y in *min_y..=*max_y {
      res.push('\n');
      for x in *min_x..=*max_x {
        if dots.contains(&(x, y)) {
          res.push('█');
        } else {
          res.push('░');
        }
      }
    }
    res
  }
}
