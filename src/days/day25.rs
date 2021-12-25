use crate::parser::grid;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map as pmap;
use nom::IResult;

use crate::days::Day;

pub struct Day25;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
  South,
  East,
  Empty,
}

impl Tile {
  fn is_empty(&self) -> bool {
    match self {
      Empty => true,
      _ => false,
    }
  }

  fn is_east(&self) -> bool {
    match self {
      East => true,
      _ => false,
    }
  }

  fn is_south(&self) -> bool {
    match self {
      South => true,
      _ => false,
    }
  }
}
use Tile::*;

fn parse_tile(input: &str) -> IResult<&str, Tile> {
  alt((
    pmap(tag("v"), |_| South),
    pmap(tag(">"), |_| East),
    pmap(tag("."), |_| Empty),
  ))(input)
}

fn run(input: &Vec<Vec<Tile>>, i: usize) -> usize {
  let mut moved = false;
  let mut output = input.clone();
  let height = input.len();
  let width = input[0].len();
  for y in 0..height {
    for x in 0..width {
      match input[y][x] {
        East => {
          if input[y][(x + 1) % width].is_empty() {
            output[y][(x + 1) % width] = East;
            output[y][x] = Empty;
            moved = true;
          }
        }
        _ => {}
      }
    }
  }
  for y in 0..height {
    for x in 0..width {
      match input[y][x] {
        South => {
          let y_target = (y + 1) % height;
          if output[y_target][x].is_empty() && !input[y_target][x].is_south() {
            output[y_target][x] = South;
            output[y][x] = Empty;
            moved = true;
          }
        }
        _ => {}
      }
    }
  }

  if moved {
    run(&output, i + 1)
  } else {
    i
  }
}

impl Day for Day25 {
  type Input = Vec<Vec<Tile>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    grid(parse_tile)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    run(input, 1)
  }

  type Output2 = String;

  fn part_2(_input: &Self::Input) -> Self::Output2 {
    unimplemented!("part_2")
  }
}
