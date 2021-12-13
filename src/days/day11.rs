use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::IResult;
use std::cmp::min;

use crate::days::Day;

pub struct Day11;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn parse_line(input: &str) -> IResult<&str, [Octopus; WIDTH]> {
  let (cont, digits_str) = digit1(input)?;
  let res: Vec<Octopus> = digits_str
    .chars()
    .map(|c| Octopus {
      level: c.to_digit(10).unwrap(),
      has_flashed: false,
    })
    .collect();
  Ok((cont, res.try_into().unwrap()))
}

#[derive(Debug, Clone)]
pub struct Octopus {
  level: u32,
  has_flashed: bool,
}

fn get_min_1(i: usize) -> usize {
  match i {
    0 => 0,
    _ => i - 1,
  }
}

fn flash(input: &mut [[Octopus; WIDTH]; HEIGHT], x: usize, y: usize) {
  input[y][x].has_flashed = true;
  for y2 in get_min_1(y)..=min(HEIGHT - 1, y + 1) {
    for x2 in get_min_1(x)..=min(WIDTH - 1, x + 1) {
      if !(y2 == y && x2 == x) {
        input[y2][x2].level += 1;
        if input[y2][x2].level > 9 && !input[y2][x2].has_flashed {
          flash(input, x2, y2);
        }
      }
    }
  }
}

fn step(input: &mut [[Octopus; WIDTH]; HEIGHT]) -> u32 {

  for row in &mut *input {
    for l in row {
      l.level += 1;
    }
  }

  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      if input[y][x].level > 9 && !input[y][x].has_flashed {
        flash(input, x, y);
      }
    }
  }

  let mut flash_count = 0;
  for row in input {
    for l in row {
      if l.has_flashed {
        l.level = 0;
        flash_count += 1;
        l.has_flashed = false;
      }
    }
  }
  flash_count
}

impl Day for Day11 {
  type Input = [[Octopus; WIDTH]; HEIGHT];

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, arrs) = separated_list1(newline, parse_line)(input)?;
    Ok((cont, arrs.try_into().unwrap()))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut input: Self::Input = input.clone();
    (0..100).fold(0, |acc, _| acc + step(&mut input))
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut input: Self::Input = input.clone();
    let mut s = 0;
    loop {
      s += 1;
      let flashes = step(&mut input);
      if flashes as usize == WIDTH * HEIGHT {
        return s;
      }
    }
  }
}
