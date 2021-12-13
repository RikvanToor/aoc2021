use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1};
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::{HashMap, HashSet};

use crate::days::Day;

pub struct Day08;

fn parse_line(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
  let (cont, (i, _, o)) = tuple((
    separated_list0(space1, alpha1),
    tag(" | "),
    separated_list0(space1, alpha1),
  ))(input)?;
  Ok((
    cont,
    (
      i.iter().map(|x| String::from(*x)).collect(),
      o.iter().map(|x| String::from(*x)).collect(),
    ),
  ))
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Copy)]
enum Segment {
  Top,
  Center,
  Bottom,
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

fn all_segments() -> HashSet<Segment> {
  use Segment::*;
  HashSet::from([
    Top,
    Center,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
  ])
}

fn update_map(
  char_map: &mut HashMap<char, HashSet<Segment>>,
  nr: &HashSet<Segment>,
  word: &str,
) {
  let inv: HashSet<Segment> = all_segments().difference(nr).cloned().collect();
  for c in 'a'..='g' {
    let cur = char_map.entry(c).or_insert_with(all_segments);
    if word.contains(c) {
      *cur = cur.intersection(nr).cloned().collect();
    } else {
      *cur = cur.intersection(&inv).cloned().collect();
    }
  }
}

fn set_chars(chars: &mut HashSet<char>, word: &str) {
  for c in word.chars() {
    chars.insert(c);
  }
}

fn find_match(input_set: &HashSet<Segment>, references: &[HashSet<Segment>]) -> i32 {
  for i in 0..=9 {
    if input_set.eq(&references[i]) {
      return i as i32;
    }
  }

  panic!("No solution found")
}

fn run_line(i: &[String], o: &[String]) -> i32 {
  use Segment::*;

  let mut char_map: HashMap<char, HashSet<Segment>> = HashMap::new();
  for x in 'a'..='g' {
    char_map.insert(x, all_segments());
  }

  let one = HashSet::from([TopRight, BottomRight]);
  let two = HashSet::from([Top, Center, Bottom, TopRight, BottomLeft]);
  let three = HashSet::from([Top, Center, Bottom, TopRight, BottomRight]);
  let four = HashSet::from([Center, TopLeft, TopRight, BottomRight]);
  let five = HashSet::from([Top, Center, Bottom, TopLeft, BottomRight]);
  let six = HashSet::from([Top, Center, Bottom, TopLeft, BottomLeft, BottomRight]);
  let seven = HashSet::from([Top, TopRight, BottomRight]);
  let eight = all_segments();
  let nine = HashSet::from([Top, Center, Bottom, TopLeft, TopRight, BottomRight]);
  let zero = HashSet::from([Top, Bottom, TopLeft, TopRight, BottomLeft, BottomRight]);
  let nrs = vec![zero, one, two, three, four, five, six, seven, eight, nine];
  let mut one_chars: HashSet<char> = HashSet::new();
  let mut four_chars: HashSet<char> = HashSet::new();
  let mut seven_chars: HashSet<char> = HashSet::new();
  for x in i {
    match x.len() {
      2 => {
        update_map(&mut char_map, &nrs[1], x);
        set_chars(&mut one_chars, x);
      }
      3 => {
        update_map(&mut char_map, &nrs[7], x);
        set_chars(&mut seven_chars, x);
      }
      4 => {
        update_map(&mut char_map, &nrs[4], x);
        set_chars(&mut four_chars, x);
      }
      _ => (),
    }
  }
  for x in i {
    let mut char_set = HashSet::new();
    for c in x.chars() {
      char_set.insert(c);
    }
    match x.len() {
      5 => {
        if char_set.intersection(&one_chars).count() == 2 {
          update_map(&mut char_map, &nrs[3], x);
        } else if char_set.intersection(&four_chars).count() == 3 {
          update_map(&mut char_map, &nrs[5], x);
        } else {
          update_map(&mut char_map, &nrs[2], x);
        }
      }
      6 => {
        if char_set.intersection(&four_chars).count() == 4 {
          update_map(&mut char_map, &nrs[9], x);
        } else if char_set.intersection(&seven_chars).count() == 3 {
          update_map(&mut char_map, &nrs[0], x);
        } else {
          update_map(&mut char_map, &nrs[6], x);
        }
      }
      _ => (),
    }
  }
  let res: i32 = o
    .iter()
    .map(|o1| {
      let mut o1_set = HashSet::new();
      for c in o1.chars() {
        for x in &char_map[&c] {
          o1_set.insert(*x);
        }
      }
      find_match(&o1_set, &nrs)
    })
    .fold(0, |acc, x| acc * 10 + x);

  res
}

impl Day for Day08 {
  type Input = Vec<(Vec<String>, Vec<String>)>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(newline, parse_line)(input)
  }

  type Output1 = i32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    input
      .iter()
      .map(|(_, o)| {
        o.iter()
          .filter(|x| {
            let l = x.len();
            l == 2 || l == 3 || l == 4 || l == 7
          })
          .count() as i32
      })
      .sum::<i32>()
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    input.iter().map(|(i,o)| run_line(i,o)).sum::<i32>()
  }
}
