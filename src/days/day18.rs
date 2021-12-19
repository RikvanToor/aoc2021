use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::combinator::map as pmap;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;

use crate::days::Day;

#[derive(Debug, Clone)]
pub enum SnailfishNumber {
  Num(i32),
  Pair(Box<Self>, Box<Self>),
}

fn parse_num(input: &str) -> IResult<&str, SnailfishNumber> {
  pmap(i32, SnailfishNumber::Num)(input)
}

fn parse_pair(input: &str) -> IResult<&str, SnailfishNumber> {
  let (cont, (n1, n2)) = delimited(
    tag("["),
    separated_pair(parse_snailfish_number, tag(","), parse_snailfish_number),
    tag("]"),
  )(input)?;
  Ok((cont, SnailfishNumber::Pair(Box::new(n1), Box::new(n2))))
}

fn parse_snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
  alt((parse_num, parse_pair))(input)
}

fn add_first_left(input: &mut SnailfishNumber, n: i32) {
  use SnailfishNumber::*;
  match input {
    Num(x) => *input = Num(*x + n),
    Pair(bn1, _) => {
      add_first_left(&mut *bn1, n);
    }
  }
}

fn add_first_right(input: &mut SnailfishNumber, n: i32) {
  use SnailfishNumber::*;
  match input {
    Num(x) => *input = Num(*x + n),
    Pair(_, bn2) => {
      add_first_right(bn2, n);
    }
  }
}

fn explode_1(input: &mut SnailfishNumber, depth: usize) -> Option<(Option<i32>, Option<i32>)> {
  use SnailfishNumber::*;
  match input {
    Pair(bn1, bn2) => {
      if let (Num(x), Num(y)) = (bn1.as_ref(), bn2.as_ref()) {
        if depth >= 4 {
          let res = Some((Some(*x), Some(*y)));
          *input = Num(0);
          res
        } else {
          None
        }
      } else if let Some((opt_add_left, opt_add_right)) = explode_1(&mut *bn1, depth + 1) {
        // The left element in the pair has had an explosion inside
        if let Some(add_right) = opt_add_right {
          add_first_left(&mut *bn2, add_right);
        }
        Some((opt_add_left, None))
      } else if let Some((opt_add_left, opt_add_right)) = explode_1(&mut *bn2, depth + 1) {
        // The right element in the pair has had an explosion inside
        if let Some(add_left) = opt_add_left {
          add_first_right(&mut *bn1, add_left);
        }
        Some((None, opt_add_right))
      } else {
        // No explosions inside this pair
        None
      }
    }
    Num(_) => None,
  }
}

fn split(input: &mut SnailfishNumber) -> bool {
  use SnailfishNumber::*;
  match input {
    Num(x) => {
      if *x >= 10 {
        let left = *x / 2;
        let right = (*x + 1) / 2;
        *input = Pair(Box::new(Num(left)), Box::new(Num(right)));
        true
      } else {
        false
      }
    }
    Pair(bn1, bn2) => split(bn1) || split(bn2),
  }
}

fn step(input: &mut SnailfishNumber) -> bool {
  explode_1(input, 0).is_some() || split(input)
}

fn reduce(input: &mut SnailfishNumber) {
  let after_step = step(input);
  if after_step {
    reduce(input);
  }
}

fn add(x1: &SnailfishNumber, x2: &SnailfishNumber) -> SnailfishNumber {
  use SnailfishNumber::*;
  let mut new_pair = Pair(Box::new(x1.clone()), Box::new(x2.clone()));
  reduce(&mut new_pair);
  new_pair
}

fn calculate_magnitude(input: &SnailfishNumber) -> i32 {
  use SnailfishNumber::*;
  match input {
    Num(x) => *x,
    Pair(l, r) => 3 * calculate_magnitude(&*l.clone()) + 2 * calculate_magnitude(&*r.clone()),
  }
}

fn sum_slice(input: &[SnailfishNumber]) -> SnailfishNumber {
  use SnailfishNumber::*;
  input
    .iter()
    .fold(None, |res, x| match res {
      None => Some(x.clone()),
      Some(res) => Some(add(&res, x)),
    })
    .unwrap_or(Num(0))
}

pub struct Day18;

impl Day for Day18 {
  type Input = Vec<SnailfishNumber>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_snailfish_number)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let res = sum_slice(input);
    calculate_magnitude(&res)
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let pairs = input.iter().combinations(2);
    let mut max = 0;
    for p in pairs {
      if let [x1, x2] = p[0..=1] {
        let mag1 = calculate_magnitude(&add(x1, x2));
        let mag2 = calculate_magnitude(&add(x2, x1));
        if mag1 > max {
          max = mag1;
        }
        if mag2 > max {
          max = mag2;
        }
      } else {
        panic!("Pairs element was not actually a pair")
      }
    }

    max
  }
}
