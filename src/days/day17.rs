use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::Ordering::*;

use crate::days::Day;

pub struct Day17;

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
  let (cont, (min, _, max)) = tuple((i32, tag(".."), i32))(input)?;

  Ok((cont, (min, max)))
}

#[derive(Debug)]
pub struct State {
  pos: (i32, i32),
  velocity: (i32, i32),
}

pub struct Trench {
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
}

fn step(s: &mut State) {
  let (x, y) = s.pos;
  let (dx, dy) = s.velocity;
  let new_dx = match dx.cmp(&0) {
    Less => dx + 1,
    Greater => dx - 1,
    Equal => dx,
  };
  s.pos = (x + dx, y + dy);
  s.velocity = (new_dx, dy - 1);
}

#[derive(Debug)]
pub enum Missed {
  Left,
  Right,
  Through,
}

fn check_velocity(t: &Trench, velocity: (i32, i32)) -> Result<i32, Missed> {
  let pos: (i32, i32) = (0, 0);
  let mut s = State { pos, velocity };
  let mut highest_y = 0;
  loop {
    if s.pos.1 > highest_y {
      highest_y = s.pos.1;
    }
    if s.pos.0 >= t.min_x && s.pos.0 <= t.max_x && s.pos.1 >= t.min_y && s.pos.1 <= t.max_y {
      return Ok(highest_y);
    } else if s.pos.1 < t.min_y && s.velocity.1 <= 0 {
      if s.pos.0 < t.min_x {
        return Err(Missed::Left);
      } else if s.pos.0 > t.max_x {
        return Err(Missed::Right);
      } else {
        return Err(Missed::Through);
      }
    } else {
      step(&mut s);
    }
  }
}

fn solve(t: &Trench) -> (i32, i32) {
  let x_start = 1;
  let x_end = t.max_x;
  let mut highest_y = 0;
  let mut count = 0;
  for x in x_start..=x_end {
    for y in t.min_y..i32::abs(t.min_y) {
      match check_velocity(t, (x, y)) {
        Ok(high_y) => {
          if high_y > highest_y {
            highest_y = high_y;
          }
          count += 1;
        }
        Err(Missed::Right) => break,
        _ => {}
      }
    }
  }
  (highest_y, count)
}

impl Day for Day17 {
  type Input = Trench;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, _) = tag("target area: x=")(input)?;
    let (cont, (min_x, max_x)) = parse_range(cont)?;
    let (cont, _) = tag(", y=")(cont)?;
    let (cont, (min_y, max_y)) = parse_range(cont)?;
    Ok((
      cont,
      Trench {
        min_x,
        max_x,
        min_y,
        max_y,
      },
    ))
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    solve(input).0
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    solve(input).1
  }
}
