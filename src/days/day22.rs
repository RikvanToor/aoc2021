use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::newline;
use nom::combinator::map as pmap;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashMap;

use crate::days::Day;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OnOff {
  On,
  Off,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
  action: OnOff,
  min_x: i64,
  max_x: i64,
  min_y: i64,
  max_y: i64,
  min_z: i64,
  max_z: i64,
}

use OnOff::*;

fn parse_line(input: &str) -> IResult<&str, Instruction> {
  let (cont, action) = alt((pmap(tag("on"), |_| On), pmap(tag("off"), |_| Off)))(input)?;
  let (cont, (_, min_x, _, max_x)) = tuple((tag(" x="), i64, tag(".."), i64))(cont)?;
  let (cont, (_, min_y, _, max_y)) = tuple((tag(",y="), i64, tag(".."), i64))(cont)?;
  let (cont, (_, min_z, _, max_z)) = tuple((tag(",z="), i64, tag(".."), i64))(cont)?;
  Ok((
    cont,
    Instruction {
      action,
      min_x,
      max_x,
      min_y,
      max_y,
      min_z,
      max_z,
    },
  ))
}

fn has_overlap(on: &Instruction, off: &Instruction) -> bool {
  off.max_x >= on.min_x
    && off.min_x <= on.max_x
    && off.max_y >= on.min_y
    && off.min_y <= on.max_y
    && off.max_z >= on.min_z
    && off.min_z <= on.max_z
}

fn push_valid(res: &mut Vec<Instruction>, i: Instruction) {
  if i.max_x >= i.min_x
    && i.max_y >= i.min_y
    && i.max_z >= i.min_z
  {
    res.push(i);
  }
}

fn split_cuboid(a: &Instruction, b: &Instruction) -> Vec<Instruction> {
  if has_overlap(a, b) {
    let mut res = vec![];
    let xs = [
      (a.min_x, b.min_x - 1),
      (max(a.min_x, b.min_x), min(a.max_x, b.max_x)),
      (b.max_x + 1, a.max_x),
    ];
    let ys = [
      (a.min_y, b.min_y - 1),
      (max(a.min_y, b.min_y), min(a.max_y, b.max_y)),
      (b.max_y + 1, a.max_y),
    ];
    let zs = [
      (a.min_z, b.min_z - 1),
      (max(a.min_z, b.min_z), min(a.max_z, b.max_z)),
      (b.max_z + 1, a.max_z),
    ];
    for x in xs {
      for y in ys {
        for z in zs {
          if !(x == xs[1] && y == ys[1] && z == zs[1]) {
            push_valid(
              &mut res,
              Instruction {
                action: On,
                min_x: x.0,
                max_x: x.1,
                min_y: y.0,
                max_y: y.1,
                min_z: z.0,
                max_z: z.1,
              },
            );
          }
        }
      }
    }
    res
  } else {
    vec![*a]
  }
}

fn run_instructions(insts: &[Instruction]) -> Vec<Instruction> {
  let mut total_instructions = vec![];
  for i in insts {
    total_instructions = total_instructions
      .iter()
      .map(|i2| split_cuboid(i2, i))
      .flatten()
      .collect();
    if let On = i.action {
      total_instructions.push(*i);
    }
  }
  total_instructions
}

pub struct Day22;

impl Day for Day22 {
  type Input = Vec<Instruction>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_line)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut hm: HashMap<(i64, i64, i64), OnOff> = HashMap::new();
    for i in input {
      for x in max(i.min_x, -50)..=min(i.max_x, 50) {
        for y in max(i.min_y, -50)..=min(i.max_y, 50) {
          for z in max(i.min_z, -50)..=min(i.max_z, 50) {
            hm.insert((x, y, z), i.action);
          }
        }
      }
    }
    hm.iter().filter(|(_, s)| **s == On).count()
  }

  type Output2 = i64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let split_insts = run_instructions(input);
    split_insts
      .iter()
      .map(|i| {
        ((i.max_x - i.min_x + 1) * (i.max_y - i.min_y + 1) * (i.max_z - i.min_z + 1)) as i64
      })
      .sum::<i64>()
  }
}
