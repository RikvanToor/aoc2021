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

fn add_first_left(input: SnailfishNumber, n: i32) -> SnailfishNumber {
  use SnailfishNumber::*;
  match input {
    Num(x) => Num(x + n),
    Pair(bn1, bn2) => {
      let new_n1 = add_first_left(*bn1, n);
      Pair(Box::new(new_n1), bn2)
    }
  }
}

fn add_first_right(input: SnailfishNumber, n: i32) -> SnailfishNumber {
  use SnailfishNumber::*;
  match input {
    Num(x) => Num(x + n),
    Pair(bn1, bn2) => {
      let new_n2 = add_first_right(*bn2, n);
      Pair(bn1, Box::new(new_n2))
    }
  }
}

fn explode_1(
  input: &SnailfishNumber,
  depth: usize,
) -> Result<(SnailfishNumber, (Option<i32>, Option<i32>)), SnailfishNumber> {
  use SnailfishNumber::*;
  match input {
    Pair(bn1, bn2) => {
      if let (Num(x), Num(y)) = (*bn1.clone(), *bn2.clone()) {
        if depth >= 4 {
          Ok((Num(0), (Some(x), Some(y))))
        } else {
          Err(input.clone())
        }
      } else {
        if let Ok((new_n1, (opt_add_left, opt_add_right))) = explode_1(&*bn1.clone(), depth + 1) {
          // The left element in the pair has had an explosion inside
          if let Some(add_right) = opt_add_right {
            let new_n2 = add_first_left(*bn2.clone(), add_right);
            Ok((
              Pair(Box::new(new_n1), Box::new(new_n2)),
              (opt_add_left, None),
            ))
          } else {
            Ok((Pair(Box::new(new_n1), bn2.clone()), (opt_add_left, None)))
          }
        } else if let Ok((new_n2, (opt_add_left, opt_add_right))) =
          explode_1(&*bn2.clone(), depth + 1)
        {
          // The right element in the pair has had an explosion inside
          if let Some(add_left) = opt_add_left {
            let new_n1 = add_first_right(*bn1.clone(), add_left);
            Ok((
              Pair(Box::new(new_n1), Box::new(new_n2)),
              (None, opt_add_right),
            ))
          } else {
            Ok((Pair(bn1.clone(), Box::new(new_n2)), (None, opt_add_right)))
          }
        } else {
          // No explosions inside this pair
          Err(input.clone())
        }
      }
    }
    Num(x) => Err(Num(*x)),
  }
}

fn split(input: &SnailfishNumber) -> Result<SnailfishNumber, SnailfishNumber> {
  use SnailfishNumber::*;
  match input {
    Num(x) => {
      if *x >= 10 {
        let left = *x / 2;
        let right = (*x + 1) / 2;
        let pair = Pair(Box::new(Num(left)), Box::new(Num(right)));
        Ok(pair)
      } else {
        Err(Num(*x))
      }
    }
    Pair(bn1, bn2) => {
      if let Ok(new_n1) = split(&*bn1) {
        Ok(Pair(Box::new(new_n1), bn2.clone()))
      } else if let Ok(new_n2) = split(&*bn2) {
        Ok(Pair(bn1.clone(), Box::new(new_n2)))
      } else {
        Err(input.clone())
      }
    }
  }
}

fn step(input: &SnailfishNumber) -> Result<SnailfishNumber, SnailfishNumber> {
  if let Ok((res, _)) = explode_1(input, 0) {
    Err(res)
  } else if let Ok(res) = split(input) {
    Err(res)
  } else {
    Ok(input.clone())
  }
}

fn reduce(input: &SnailfishNumber) -> SnailfishNumber {
  match step(input) {
    Ok(res) => res,
    Err(res) => reduce(&res),
  }
}

fn add(x1: &SnailfishNumber, x2: &SnailfishNumber) -> SnailfishNumber {
  use SnailfishNumber::*;
  let new_pair = Pair(Box::new(x1.clone()), Box::new(x2.clone()));
  reduce(&new_pair)
}

fn print_num(input: &SnailfishNumber) -> String {
  use SnailfishNumber::*;
  match input {
    Num(x) => x.to_string(),
    Pair(l, r) => {
      format!("[{},{}]", print_num(&*l.clone()), print_num(&*r.clone()))
    }
  }
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
