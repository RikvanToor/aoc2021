use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::newline;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::days::Day;

pub struct Day19;

type Pos = (i32, i32, i32);

#[derive(Clone)]
pub struct Scanner {
  points: HashSet<Pos>,
  distances: HashSet<i32>,
}

fn parse_vec3(input: &str) -> IResult<&str, Pos> {
  let (cont, (x, _, y, _, z)) = tuple((i32, tag(","), i32, tag(","), i32))(input)?;
  Ok((cont, (x, y, z)))
}

fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
  let (cont, _) = tuple((tag("--- scanner "), i32, tag(" ---"), newline))(input)?;
  let (cont, poss) = separated_list0(newline, parse_vec3)(cont)?;
  let points = HashSet::from_iter(poss);
  let distances = calculate_distances(&points);
  Ok((cont, Scanner { points, distances }))
}

fn calculate_distances(points: &HashSet<Pos>) -> HashSet<i32> {
  points
    .iter()
    .combinations(2)
    .map(|ps| {
      let dx = ps[0].0 - ps[1].0;
      let dy = ps[0].1 - ps[1].1;
      let dz = ps[0].2 - ps[1].2;
      let dist = (((dx * dx + dy * dy + dz * dz) as f32).sqrt() * 100000.0).round() as i32;
      dist
    })
    .collect()
}

fn min3((x1, y1, z1): &Pos, (x2, y2, z2): &Pos) -> Pos {
  (x1 - x2, y1 - y2, z1 - z2)
}

enum Axis {
  X,
  Y,
  Z,
}

enum SignedAxis {
  Pos(Axis),
  Neg(Axis),
}

fn get_unsigned_value(a: &Axis, pos: &Pos) -> i32 {
  match a {
    Axis::X => pos.0,
    Axis::Y => pos.1,
    Axis::Z => pos.2,
  }
}

fn get_value(sa: &SignedAxis, pos: &Pos) -> i32 {
  match sa {
    SignedAxis::Pos(a) => get_unsigned_value(a, pos),
    SignedAxis::Neg(a) => -get_unsigned_value(a, pos),
  }
}

type Rotation = (SignedAxis, SignedAxis, SignedAxis);

fn get_all_rotations() -> Vec<Rotation> {
  use Axis::*;
  use SignedAxis::*;
  vec![
    (Pos(X), Pos(Y), Pos(Z)),
    (Pos(X), Neg(Y), Neg(Z)),
    (Neg(X), Pos(Y), Neg(Z)),
    (Neg(X), Neg(Y), Pos(Z)),
    (Pos(X), Pos(Z), Neg(Y)),
    (Pos(X), Neg(Z), Pos(Y)),
    (Neg(X), Pos(Z), Pos(Y)),
    (Neg(X), Neg(Z), Neg(Y)),
    (Pos(Y), Pos(Z), Pos(X)),
    (Pos(Y), Neg(Z), Neg(X)),
    (Neg(Y), Pos(Z), Neg(X)),
    (Neg(Y), Neg(Z), Pos(X)),
    (Pos(Y), Pos(X), Neg(Z)),
    (Pos(Y), Neg(X), Pos(Z)),
    (Neg(Y), Pos(X), Pos(Z)),
    (Neg(Y), Neg(X), Neg(Z)),
    (Pos(Z), Pos(X), Pos(Y)),
    (Pos(Z), Neg(X), Neg(Y)),
    (Neg(Z), Pos(X), Neg(Y)),
    (Neg(Z), Neg(X), Pos(Y)),
    (Pos(Z), Pos(Y), Neg(X)),
    (Pos(Z), Neg(Y), Pos(X)),
    (Neg(Z), Pos(Y), Pos(X)),
    (Neg(Z), Neg(Y), Neg(X)),
  ]
}

fn rotate((rx, ry, rz): &Rotation, pos: &Pos) -> Pos {
  (get_value(rx, pos), get_value(ry, pos), get_value(rz, pos))
}

fn find_overlap(scanner0: &Scanner, scanner1: &Scanner) -> Option<(Pos, Scanner)> {
  let all_rotations = get_all_rotations();
  if scanner0.distances.intersection(&scanner1.distances).count() >= 56 {
    for p0 in scanner0.points.iter().skip(11) {
      for p1 in scanner1.points.iter().skip(11) {
        for rotation in &all_rotations {
          let p_base = rotate(rotation, &p1);
          let p_diff = min3(&p_base, &p0);
          let transformed_slice = scanner1.points.iter().map(|p| {
            let p_rotated = rotate(rotation, p);
            min3(&p_rotated, &p_diff)
          });
          if transformed_slice
            .clone()
            .filter(|p| scanner0.points.contains(p))
            .count()
            >= 12
          {
            let points = transformed_slice.collect();
            let distances = scanner1.distances.clone();
            return Some((min3(&(0, 0, 0), &p_diff), Scanner { points, distances }));
          }
        }
      }
    }
  }
  None
}

fn manhattan_distance(p1: &Pos, p2: &Pos) -> i32 {
  i32::abs(p2.0 - p1.0) + i32::abs(p2.1 - p1.1) + i32::abs(p2.2 - p1.2)
}

fn helper(
  total_scanner: &mut Scanner,
  scanner_positions: &mut Vec<Pos>,
  scanner_list: &[Scanner],
) -> Vec<Scanner> {
  let mut res = vec![];
  for s in scanner_list {
    match find_overlap(total_scanner, s) {
      None => res.push(s.clone()),
      Some((scanner_pos, s1)) => {
        scanner_positions.push(scanner_pos);
        total_scanner.points.extend(&s1.points);
        total_scanner.distances = calculate_distances(&total_scanner.points);
      }
    }
  }
  res
}

fn combine_all(input: &[Scanner]) -> (usize, Vec<Pos>) {
  let mut start_scanner = input[0].clone();
  let mut list: Vec<Scanner> = input[1..].to_vec();
  let mut positions: Vec<Pos> = vec![(0, 0, 0)];
  while !list.is_empty() {
    let new_list = helper(&mut start_scanner, &mut positions, &list);
    list = new_list;
  }
  (start_scanner.points.len(), positions)
}

impl Day for Day19 {
  type Input = Vec<Scanner>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    separated_list0(pair(newline, newline), parse_scanner)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (res, _) = combine_all(input);
    res
  }

  type Output2 = i32;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (_, positions) = combine_all(input);
    positions
      .iter()
      .combinations(2)
      .map(|xs| manhattan_distance(xs[0], xs[1]))
      .max()
      .unwrap()
  }
}
