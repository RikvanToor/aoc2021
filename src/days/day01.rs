use nom::character::complete::{i32, newline};
use nom::multi::separated_list0;
use nom::IResult;

use crate::days::Day;

pub struct Day01;

fn helper(input: &[i32]) -> i32 {
  let mut counter: i32 = 0;
  for i in 1..input.len() {
    if input[i] > input[i - 1] {
      counter += 1;
    }
  }
  counter
}

impl Day for Day01 {
  type Input = Vec<i32>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, i32)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    helper(input)
  }


  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut tmp = vec![];
    for i in 1..input.len()-1 {
      tmp.push(input[i-1]+input[i]+input[i+1])
    }
    helper(&tmp)
  }
}
