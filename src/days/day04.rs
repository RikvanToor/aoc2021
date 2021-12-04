use nom::character::complete::{char, i32, newline, space0};
use nom::combinator::{map as pmap, opt};
use nom::multi::{count, many1, separated_list0};
use nom::sequence::pair;
use nom::IResult;
use std::convert::TryInto;

use crate::days::Day;

pub struct Day04;

fn parse_row(input: &str) -> IResult<&str, [i32; 5]> {
  let (cont, res) = count(pmap(pair(space0, i32), |x| x.1), 5)(input)?;
  let arr = res.try_into().unwrap();
  Ok((cont, arr))
}

fn parse_board(input: &str) -> IResult<&str, [[i32; 5]; 5]> {
  let (cont, res) = count(pmap(pair(parse_row, opt(newline)), |x| x.0), 5)(input)?;
  Ok((cont, res.try_into().unwrap()))
}

fn setup_boards(boards: &Vec<[[i32; 5]; 5]>) -> Vec<[[(i32, bool); 5]; 5]> {
  boards
    .iter()
    .map(|b| b.map(|r| r.map(|x| (x, false))))
    .collect()
}

fn update_boards(boards: &mut Vec<[[(i32, bool); 5]; 5]>, n: i32) {
  for bi in 0..boards.len() {
    for x in 0..5 {
      for y in 0..5 {
        if boards[bi][x][y].0 == n {
          boards[bi][x][y].1 = true;
        }
      }
    }
  }
}

fn has_won(board: &[[(i32, bool); 5]; 5]) -> bool {
  let mut bi = board.iter();
  let transposed: [[(i32, bool); 5]; 5] = [0, 1, 2, 3, 4].map(|i| board.map(|r| r[i]));
  let mut ti = transposed.iter();
  bi.any(|r| r.iter().all(|x| x.1)) || ti.any(|c| c.iter().all(|x| x.1))
}

fn non_marked_sum(board: &[[(i32, bool); 5]; 5]) -> i32 {
  board
    .iter()
    .map(|r| r.iter().filter(|x| !x.1).map(|x| x.0).sum::<i32>())
    .sum()
}

impl Day for Day04 {
  type Input = (Vec<i32>, Vec<[[i32; 5]; 5]>);

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, (nums, _)) = pair(separated_list0(char(','), i32), count(newline, 2))(input)?;
    let (cont, boards) = separated_list0(many1(newline), parse_board)(cont)?;
    Ok((cont, (nums, boards)))
  }

  type Output1 = i32;

  fn part_1((nums, boards): &Self::Input) -> Self::Output1 {
    let mut boards_results = setup_boards(boards);

    for n in nums {
      update_boards(&mut boards_results, *n);

      for b in &boards_results {
        if has_won(b) {
          return non_marked_sum(b) * n;
        }
      }
    }

    panic!("No full board found")
  }

  type Output2 = i32;

  fn part_2((nums, boards): &Self::Input) -> Self::Output2 {
    let mut boards_results = setup_boards(boards);

    for n in nums {
      update_boards(&mut boards_results, *n);

      if boards_results.len() > 1 {
        boards_results = boards_results
          .into_iter()
          .filter(|b| !has_won(b))
          .collect();
      } else if has_won(&boards_results[0]) {
        return non_marked_sum(&boards_results[0]) * n;
      }
    }

    panic!("No full board found")
  }
}
