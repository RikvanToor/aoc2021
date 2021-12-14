use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, newline};
use nom::multi::{many1, separated_list0};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

use crate::days::Day;

pub struct Day14;

type Rule = ((char, char), char);

fn parse_rule(input: &str) -> IResult<&str, Rule> {
  let (cont, (l1, l2, _, r)) = tuple((anychar, anychar, tag(" -> "), anychar))(input)?;
  Ok((cont, ((l1, l2), r)))
}

type AlgorithmInputs = (
  HashMap<(char, char), usize>,
  HashMap<(char, char), char>,
  char,
);

fn build_inputs(init: &str, rules: &[Rule]) -> AlgorithmInputs {
  let rules: HashMap<(char, char), char> = rules.iter().cloned().collect();
  let mut occs: HashMap<(char, char), usize> = HashMap::new();
  let init: Vec<char> = init.chars().collect();
  let windows = init.windows(2);
  for w in windows {
    *occs.entry((w[0], w[1])).or_default() += 1;
  }

  (occs, rules, init[0])
}

fn run(steps: usize, init: &str, rules: &[Rule]) -> usize {
  let (init, rules, first_char) = build_inputs(init, rules);
  let new_occs = (0..steps).fold(init, |acc, _| step(&acc, &rules));

  get_answer(&build_counts(&new_occs, first_char))
}

fn step(
  init: &HashMap<(char, char), usize>,
  rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
  let mut occs: HashMap<(char, char), usize> = HashMap::new();
  for (k, c) in init {
    let (l1, l2) = k;
    if let Some(r) = rules.get(k) {
      *occs.entry((*l1, *r)).or_default() += c;
      *occs.entry((*r, *l2)).or_default() += c;
    }
  }
  occs
}

fn build_counts(occs: &HashMap<(char, char), usize>, first_char: char) -> HashMap<char, usize> {
  let mut res = HashMap::new();
  res.insert(first_char, 1);
  for ((_, c2), count) in occs {
    *res.entry(*c2).or_default() += count;
  }
  res
}

fn get_answer(counts: &HashMap<char, usize>) -> usize {
  let max = counts.iter().max_by_key(|(_, x)| *x).unwrap().1;
  let min = counts.iter().min_by_key(|(_, x)| *x).unwrap().1;
  max - min
}

impl Day for Day14 {
  type Input = (String, Vec<Rule>);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, (init, _)) = tuple((alpha1, many1(newline)))(input)?;
    let (cont, l) = separated_list0(newline, parse_rule)(cont)?;
    Ok((cont, (init.to_owned(), l)))
  }

  type Output1 = usize;

  fn part_1((init, rules): &Self::Input) -> Self::Output1 {
    run(10, init, rules)
  }

  type Output2 = usize;

  fn part_2((init, rules): &Self::Input) -> Self::Output2 {
    run(40, init, rules)
  }
}
