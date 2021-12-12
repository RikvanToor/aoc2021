use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::{HashMap, HashSet};

use crate::days::Day;

pub struct Day12;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Cave {
  Other(u32),
  Start,
  End,
}

fn str_to_cave<'a>(
  keys_map: &mut HashMap<&'a str, u32>,
  max_value: &mut u32,
  input: &'a str,
) -> Cave {
  use Cave::*;
  match input {
    "start" => Start,
    "end" => End,
    _ => {
      let is_small = input.chars().next().unwrap().is_lowercase();
      let key = match keys_map.get(input) {
        Some(k) => *k,
        None => {
          *max_value <<= 1;
          let key = *max_value + if is_small { 1 } else { 0 };
          keys_map.insert(input, key);
          key
        }
      };
      Other(key)
    }
  }
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
  let (cont, (a, _, b)) = tuple((alpha1, tag("-"), alpha1))(input)?;
  Ok((cont, (a, b)))
}

fn run(
  c: &Cave,
  hm: &HashMap<Cave, HashSet<Cave>>,
  visited: u32,
  has_repeated_small: bool,
) -> usize {
  use Cave::*;
  let nodes = hm.get(c).unwrap();
  let mut res = 0;
  for c in nodes {
    match c {
      Other(k) => {
        if k & 1 == 1 {
          // Remove small sign, just keep the ID
          let uk = k ^ 1;
          //cave is small
          if visited & uk != uk {
            //this node has not been visited
            let path = visited | uk;
            let ress = run(c, hm, path, has_repeated_small);
            res += ress;
          } else if !has_repeated_small {
            //This node has been visited, but we can repeat it
            let path = visited | uk;
            let ress = run(c, hm, path, true);
            res += ress;
          }
        } else {
          //cave is big
          let ress = run(c, hm, visited.clone(), has_repeated_small);
          res += ress;
        }
      }
      End => {
        res += 1;
      }
      Start => {}
    }
  }
  res
}

impl Day for Day12 {
  type Input = HashMap<Cave, HashSet<Cave>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, list) = separated_list0(newline, parse_line)(input)?;
    let mut keys: HashMap<&str, u32> = HashMap::new();
    let mut hm: HashMap<Cave, HashSet<Cave>> = HashMap::new();

    let mut max_value = 1;

    for (s1, s2) in list {
      let c1 = str_to_cave(&mut keys, &mut max_value, s1);
      let c2 = str_to_cave(&mut keys, &mut max_value, s2);
      let entry1 = hm.entry(c1.clone()).or_insert(HashSet::new());
      entry1.insert(c2.clone());
      let entry2 = hm.entry(c2.clone()).or_insert(HashSet::new());
      entry2.insert(c1.clone());
    }

    Ok((cont, hm))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let paths = run(&Cave::Start, input, 0, true);

    paths
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let paths = run(&Cave::Start, input, 0, false);

    paths
  }
}
