use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32, newline};
use nom::combinator::map as pmap;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::IResult;

use crate::days::Day;

pub struct Day02;

#[derive(Debug)]
pub enum Move {
  Forward(i32),
  Down(i32),
  Up(i32),
}

fn parse_move(input: &str) -> IResult<&str, Move> {
  alt((
    pmap(pair(tag("forward "), i32), |(_, x)| Move::Forward(x)),
    pmap(pair(tag("down "), i32), |(_, x)| Move::Down(x)),
    pmap(pair(tag("up "), i32), |(_, x)| Move::Up(x)),
  ))(input)
}

impl Day for Day02 {
  type Input = Vec<Move>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_move)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut x = 0;
    let mut y = 0;
    for m in input {
      match m {
        Move::Forward(d) => x += d,
        Move::Up(d) => y -= d,
        Move::Down(d) => y += d,
      }
    }
    x * y
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for m in input {
      match m {
        Move::Forward(d) => {
          x += d;
          y += aim * d;
        }
        Move::Up(d) => aim -= d,
        Move::Down(d) => aim += d,
      }
    }
    x * y
  }
}
