use nom::IResult;

use crate::days::Day;

pub struct Day10;

#[derive(Debug)]
enum Delimiter {
  Parenthesis,
  Curly,
  Brace,
  Tag,
}
use Delimiter::*;

fn check_line(line: &str) -> Result<Vec<Delimiter>, Delimiter> {
  let mut levels: Vec<Delimiter> = vec![];

  for c in line.chars() {
    match c {
      '(' => levels.push(Parenthesis),
      '{' => levels.push(Curly),
      '[' => levels.push(Brace),
      '<' => levels.push(Tag),
      ')' => match levels.pop() {
        Some(Parenthesis) => (),
        _ => return Err(Parenthesis),
      },
      '}' => match levels.pop() {
        Some(Curly) => (),
        _ => return Err(Curly),
      },
      ']' => match levels.pop() {
        Some(Brace) => (),
        _ => return Err(Brace),
      },
      '>' => match levels.pop() {
        Some(Tag) => (),
        _ => return Err(Tag),
      },
      x => panic!("Unexpected character {}", x),
    }
  }

  Ok(levels)
}

impl Day for Day10 {
  type Input = Vec<String>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    // Faking a nom parser, since we really just need to split by lines.
    let ls = input.lines().map(String::from);
    Ok(("", ls.collect()))
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|l| match check_line(l) {
        Ok(_) => 0,
        Err(Parenthesis) => 3,
        Err(Brace) => 57,
        Err(Curly) => 1197,
        Err(Tag) => 25137,
      })
      .sum::<i32>()
  }

  type Output2 = i64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut scores: Vec<i64> = input
      .iter()
      .map(|l| check_line(l))
      .filter(|x| x.is_ok())
      .map(|x| match x {
        Err(_) => 0,
        Ok(remaining) => remaining.iter().rfold(0, |acc, x| {
          acc * 5
            + match x {
              Parenthesis => 1,
              Brace => 2,
              Curly => 3,
              Tag => 4,
            }
        }),
      })
      .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
  }
}
