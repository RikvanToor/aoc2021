use crate::days::Day;
use crate::parser::grid;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map as pmap;
use nom::multi::many1;
use nom::IResult;

pub struct Day20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Pixel {
  Light,
  Dark,
}

use Pixel::*;

fn parse_pixel(input: &str) -> IResult<&str, Pixel> {
  alt((pmap(tag("#"), |_| Light), pmap(tag("."), |_| Dark)))(input)
}

fn get_index(input: &[Vec<Pixel>], px: isize, py: isize, default: Pixel) -> usize {
  let mut res = 0;
  let input_width = input[0].len() as isize;
  let input_height = input.len() as isize;
  for y in py - 1..=py + 1 {
    for x in px - 1..=px + 1 {
      let pixel = if y >= 0 && y < input_height && x >= 0 && x < input_width {
        input[y as usize][x as usize]
      } else {
        default
      };
      let digit = match pixel {
        Light => 1,
        Dark => 0,
      };
      res = (res << 1) + digit;
    }
  }

  res
}

fn step(input: &[Vec<Pixel>], algorithm: &[Pixel], default: Pixel) -> Vec<Vec<Pixel>> {
  let input_width = input[0].len() as isize;
  let input_height = input.len() as isize;
  let mut new_image = vec![];

  for y in -1..input_height + 1 {
    let mut row = vec![];
    for x in -1..input_width + 1 {
      let pixel = &algorithm[get_index(input, x, y, default)];
      row.push(*pixel);
    }
    new_image.push(row);
  }

  new_image
}

fn repeat_steps(
  input: Vec<Vec<Pixel>>,
  algorithm: &[Pixel],
  default: Pixel,
  n: usize,
  i: usize,
) -> Vec<Vec<Pixel>> {
  if i >= n {
    input
  } else {
    let new_default = match default {
      Dark => algorithm[0],
      Light => algorithm[511],
    };
    repeat_steps(
      step(&input, algorithm, default),
      algorithm,
      new_default,
      n,
      i + 1,
    )
  }
}

fn count_lights(input: &[Vec<Pixel>]) -> usize {
  input
    .iter()
    .map(|r| r.iter().filter(|x| **x == Light).count())
    .sum::<usize>()
}

fn _print_image(input: &[Vec<Pixel>]) {
  for row in input {
    for p in row {
      match p {
        Light => print!("█"),
        Dark => print!("░"),
      }
    }
    println!();
  }
}

impl Day for Day20 {
  type Input = (Vec<Pixel>, Vec<Vec<Pixel>>);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, algorithm) = many1(parse_pixel)(input)?;
    let (cont, _) = many1(newline)(cont)?;
    let (cont, image) = grid(parse_pixel)(cont)?;
    Ok((cont, (algorithm, image)))
  }

  type Output1 = usize;

  fn part_1((algorithm, image): &Self::Input) -> Self::Output1 {
    let result = repeat_steps(image.clone(), algorithm, Dark, 2, 0);
    count_lights(&result)
  }

  type Output2 = usize;

  fn part_2((algorithm, image): &Self::Input) -> Self::Output2 {
    let result = repeat_steps(image.clone(), algorithm, Dark, 50, 0);
    count_lights(&result)
  }
}
