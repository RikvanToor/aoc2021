use core::ops::Range;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::character::complete::space1;
use nom::combinator::map as pmap;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::{HashMap, HashSet};

use crate::days::Day;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Var {
  W,
  X,
  Y,
  Z,
}

#[derive(Debug, Copy, Clone)]
pub enum Val {
  Var(Var),
  Int(i32),
}

#[derive(Debug, Copy, Clone)]
pub enum Stmt {
  Inp(Var),
  Add(Var, Val),
  Mul(Var, Val),
  Div(Var, Val),
  Mod(Var, Val),
  Eql(Var, Val),
}

fn parse_var(input: &str) -> IResult<&str, Var> {
  use Var::*;
  alt((
    pmap(tag("w"), |_| W),
    pmap(tag("x"), |_| X),
    pmap(tag("y"), |_| Y),
    pmap(tag("z"), |_| Z),
  ))(input)
}

fn parse_val(input: &str) -> IResult<&str, Val> {
  alt((pmap(parse_var, Val::Var), pmap(i32, Val::Int)))(input)
}

fn parse_stmt(input: &str) -> IResult<&str, Stmt> {
  use Stmt::*;
  alt((
    pmap(tuple((tag("inp"), space1, parse_var)), |(_, _, v)| Inp(v)),
    pmap(
      tuple((tag("add"), space1, parse_var, space1, parse_val)),
      |(_, _, v1, _, v2)| Add(v1, v2),
    ),
    pmap(
      tuple((tag("mul"), space1, parse_var, space1, parse_val)),
      |(_, _, v1, _, v2)| Mul(v1, v2),
    ),
    pmap(
      tuple((tag("div"), space1, parse_var, space1, parse_val)),
      |(_, _, v1, _, v2)| Div(v1, v2),
    ),
    pmap(
      tuple((tag("mod"), space1, parse_var, space1, parse_val)),
      |(_, _, v1, _, v2)| Mod(v1, v2),
    ),
    pmap(
      tuple((tag("eql"), space1, parse_var, space1, parse_val)),
      |(_, _, v1, _, v2)| Eql(v1, v2),
    ),
  ))(input)
}

fn split_groups(input: &[Stmt]) -> Vec<Vec<Stmt>> {
  let mut res = vec![];
  let mut cur = vec![input[0].clone()];
  for s in input.iter().skip(1) {
    match s {
      Stmt::Inp(_) => {
        res.push(cur);
        cur = vec![*s];
      }
      _ => {
        cur.push(*s);
      }
    }
  }

  res.push(cur);

  res
}

fn get(memory: &HashMap<Var, i32>, val: &Val) -> i32 {
  use Val::*;
  match val {
    Var(v) => memory[v],
    Int(i) => *i,
  }
}

fn run_single_inp(program: &[Stmt], inp: i32, memory: &mut HashMap<Var, i32>) {
  use Stmt::*;
  for s in program {
    match s {
      Inp(v) => {
        memory.insert(*v, inp);
      }
      Add(v1, v2) => {
        memory.insert(*v1, memory[v1] + get(&memory, v2));
      }
      Mul(v1, v2) => {
        memory.insert(*v1, memory[v1] * get(&memory, v2));
      }
      Div(v1, v2) => {
        memory.insert(*v1, memory[v1] / get(&memory, v2));
      }
      Mod(v1, v2) => {
        memory.insert(*v1, memory[v1] % get(&memory, v2));
      }
      Eql(v1, v2) => {
        memory.insert(*v1, if memory[v1] == get(&memory, v2) { 1 } else { 0 });
      }
    }
  }
}

fn step4<I: Iterator<Item = u64> + Clone>(
  memo: &mut [HashSet<i32>; 14],
  z: i32,
  programs: &[Vec<Stmt>],
  i: usize,
  range: I,
) -> Option<Vec<u64>> {
  if memo[i].contains(&z) {
    return None;
  }
  memo[i].insert(z);

  for d in range.clone() {
    let mut memory = HashMap::new();
    memory.insert(Var::W, 0);
    memory.insert(Var::X, 0);
    memory.insert(Var::Y, 0);
    memory.insert(Var::Z, z);

    run_single_inp(&programs[i], d as i32, &mut memory);
    let new_z = memory[&Var::Z];

    if i == 13 {
      if new_z == 0 {
        return Some(vec![d]);
      } else {
        continue;
      }
    } else {
      match step4(memo, new_z, programs, i + 1, range.clone()) {
        Some(ds) => {
          let mut res = ds;
          res.insert(0, d);
          return Some(res);
        }
        None => {
          continue;
        }
      }
    }
  }

  None
}

fn run<I: Iterator<Item = u64> + Clone>(program: &[Stmt], range: I) -> u64 {
  let mut memo = [
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
    HashSet::new(),
  ];

  let programs = split_groups(program);
  let digits = step4(&mut memo, 0, &programs, 0, range.clone()).unwrap();
  let mut res = 0;
  for d in digits {
    res = res * 10 + d;
  }
  res
}

pub struct Day24;

impl Day for Day24 {
  type Input = Vec<Stmt>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_stmt)(input)
  }

  type Output1 = u64;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    run(input, (1..=9).rev())
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    run(input, 1..=9)
  }
}
