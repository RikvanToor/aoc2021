use nom::bytes::complete::tag;
use nom::character::complete::u8;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::HashMap;

use crate::days::Day;

pub struct Day06;

fn run(input: &Vec<u8>, days: u32) -> u64 {
  let mut counts: HashMap<u8, u64> = (0..=8).map(|x| (x, 0)).collect();
  for x in input {
    *counts.entry(*x).or_default() += 1;
  }

  for _ in 0..days {
    let zeroes = counts[&0];
    for x in 1..=8 {
      counts.insert(x - 1, counts[&x]);
    }
    *counts.entry(6).or_default() += zeroes;
    *counts.entry(8).or_default() = zeroes;
  }

  let res: u64 = counts.values().sum();

  res
}

impl Day for Day06 {
  type Input = Vec<u8>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(tag(","), u8)(input)
  }

  type Output1 = u64;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    run(input, 80)
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    run(input, 256)
  }
}
