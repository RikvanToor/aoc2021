use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

use crate::days::Day;

pub struct Day12;

const START: u64 = 1;
const END: u64 = 2;

fn str_to_cave<'a>(
  keys_map: &mut HashMap<&'a str, u64>,
  max_value: &mut u64,
  input: &'a str,
) -> u64 {
  match input {
    "start" => START,
    "end" => END,
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
      key
    }
  }
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
  let (cont, (a, _, b)) = tuple((alpha1, tag("-"), alpha1))(input)?;
  Ok((cont, (a, b)))
}

fn make_map_key(c: u64, visited: u64, has_repeated_small: u64) -> u64 {
  c << 25 | has_repeated_small << 24 | visited
}

fn run(
  c: &u64,
  hm: &HashMap<u64, Vec<u64>>,
  res_map: &mut HashMap<u64, usize>,
  visited: u64,
  has_repeated_small: u64, // 1 as true, 0 as false
) -> usize {
  let nodes = hm.get(c).unwrap();
  let mut res = 0;
  if let Some(x) = res_map.get(&make_map_key(*c, visited, has_repeated_small)) {
    return *x;
  }
  for c in nodes {
    match c {
      1 => {} //start
      2 => {
        //end
        res += 1;
      }
      k => {
        if k & 1 == 1 {
          // Remove small sign, just keep the ID
          let uk = k ^ 1;
          //cave is small
          if visited & uk != uk {
            //this node has not been visited
            let path = visited | uk;
            let ress = run(c, hm, res_map, path, has_repeated_small);
            res += ress;
          } else if has_repeated_small == 0 {
            //This node has been visited, but we can repeat it
            let path = visited | uk;
            let ress = run(c, hm, res_map, path, 1);
            res += ress;
          }
        } else {
          //cave is big
          let ress = run(c, hm, res_map, visited, has_repeated_small);
          res += ress;
        }
      }
    }
  }
  res_map.insert(make_map_key(*c, visited, has_repeated_small), res);
  res
}

impl Day for Day12 {
  type Input = HashMap<u64, Vec<u64>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, list) = separated_list0(newline, parse_line)(input)?;
    let mut keys: HashMap<&str, u64> = HashMap::new();
    let mut hm: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut max_value = 4;

    for (s1, s2) in list {
      let c1 = str_to_cave(&mut keys, &mut max_value, s1);
      let c2 = str_to_cave(&mut keys, &mut max_value, s2);
      let entry1 = hm.entry(c1).or_default();
      entry1.push(c2);
      let entry2 = hm.entry(c2).or_default();
      entry2.push(c1);
    }

    Ok((cont, hm))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut res_map = HashMap::new();
    run(&START, input, &mut res_map, 0, 1)
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut res_map = HashMap::new();
    run(&START, input, &mut res_map, 0, 0)
  }
}
