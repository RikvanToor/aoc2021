use itertools::sorted;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::multi::separated_list0;
use nom::IResult;

use crate::days::Day;

pub struct Day07;

impl Day for Day07 {
  type Input = Vec<i32>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(tag(","), i32)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let s: Vec<i32> = sorted(input.to_owned()).collect();
    let med = s[s.len() / 2];

    s.iter().map(|x| i32::abs(x - med)).sum::<i32>()
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let avg = input.iter().sum::<i32>() / input.iter().len() as i32;

    input
      .iter()
      .map(|x| (1..=i32::abs(x - avg)).sum::<i32>())
      .sum::<i32>()
  }
}
