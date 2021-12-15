use crate::parser::grid;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map as pmap;
use nom::IResult;

use crate::days::Day;

pub struct Day03;

enum FilterMode {
  MostCommon,
  LeastCommon,
}

fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
  (0..input[0].len())
    .map(|i| input.iter().map(|r| r[i]).collect())
    .collect()
}

fn filter_common(input: &[Vec<u32>], i: usize, mode: FilterMode) -> Vec<Vec<u32>> {
  if input.len() <= 1 {
    return input.to_owned();
  }
  let input_len = input.len() as u32;
  let count = input.iter().map(|r| r[i]).sum::<u32>();
  let filter_key = match (count * 2 >= input_len, mode) {
    (true, FilterMode::MostCommon) | (false, FilterMode::LeastCommon) => 1,
    _ => 0,
  };
  let res = input
    .iter()
    .filter(|r| r[i] == filter_key)
    .cloned()
    .collect();
  res
}

fn parse_bits(input: &[u32]) -> u32 {
  input.iter().fold(0, |acc, x| acc * 2 + x)
}

impl Day for Day03 {
  type Input = Vec<Vec<u32>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    grid(alt((pmap(char('0'), |_| 0), pmap(char('1'), |_| 1))))(input)
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let transposed: Vec<Vec<u32>> = transpose(input);
    let input_len = input.len();
    let gamma_bits: Vec<u32> = transposed
      .iter()
      .map(|r| {
        if r.iter().cloned().sum::<u32>() as usize > input_len / 2 {
          1
        } else {
          0
        }
      })
      .collect();
    let gamma: u32 = parse_bits(&gamma_bits);
    let epsilon: u32 = parse_bits(&gamma_bits.iter().map(|x| 1 - x).collect::<Vec<u32>>());
    gamma * epsilon
  }

  type Output2 = u32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let oxygen_bits = &(0..input[0].len()).fold(input.to_owned(), |acc, i| {
      filter_common(&acc, i, FilterMode::MostCommon)
    })[0];
    let co2_bits = &(0..input[0].len()).fold(input.to_owned(), |acc, i| {
      filter_common(&acc, i, FilterMode::LeastCommon)
    })[0];
    let oxygen = parse_bits(oxygen_bits);
    let co2 = parse_bits(co2_bits);
    oxygen * co2
  }
}
