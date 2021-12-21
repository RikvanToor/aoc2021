use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::character::complete::u32;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;

use crate::days::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
  position: u32,
  score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
  P1,
  P2,
}

fn next_turn(t: &Turn) -> Turn {
  use Turn::*;
  match t {
    P1 => P2,
    P2 => P1,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
  p1: Player,
  p2: Player,
  turn: Turn,
}

fn run_roll(p: &Player, roll: u32) -> Player {
  let new_pos = (p.position + roll - 1) % 10 + 1;
  Player {
    position: new_pos,
    score: p.score + new_pos,
  }
}

fn new_state(state: &State, roll: u32) -> State {
  use Turn::*;
  State {
    turn: next_turn(&state.turn),
    p1: match state.turn {
      P1 => run_roll(&state.p1, roll),
      P2 => state.p1,
    },
    p2: match state.turn {
      P1 => state.p2,
      P2 => run_roll(&state.p2, roll),
    },
    ..*state
  }
}

fn quantum_dice(state: State, memo: &mut HashMap<State, (u64, u64)>) -> (u64, u64) {
  if let Some(res) = memo.get(&state) {
    return *res;
  }

  // Winner was found in last turn
  match state.turn {
    Turn::P1 => {
      if state.p2.score >= 21 {
        memo.insert(state, (0, 1));
        return (0, 1);
      }
    }
    Turn::P2 => {
      if state.p1.score >= 21 {
        memo.insert(state, (1, 0));
        return (1, 0);
      }
    }
  }

  let possible_rolls = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

  let mut res = (0, 0);

  for (roll, amount) in possible_rolls {
    let sub_state = new_state(&state, roll);
    let (p1, p2) = quantum_dice(sub_state, memo);
    res = (res.0 + p1 * amount, res.1 + p2 * amount);
  }

  memo.insert(state, res);

  res
}

pub struct Day21;

impl Day for Day21 {
  type Input = State;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let (cont, (_, p1_pos, _)) = tuple((tag("Player 1 starting position: "), u32, newline))(input)?;
    let (cont, (_, p2_pos)) = tuple((tag("Player 2 starting position: "), u32))(cont)?;

    let p1 = Player {
      position: p1_pos,
      score: 0,
    };
    let p2 = Player {
      position: p2_pos,
      score: 0,
    };
    Ok((
      cont,
      State {
        p1,
        p2,
        turn: Turn::P1,
      },
    ))
  }

  type Output1 = u32;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let mut state = input.clone();

    let mut die_index = 0;
    let mut die_rolls = 0;

    loop {
      die_rolls += 3;
      let die_roll = die_index + (die_index + 1) % 100 + (die_index + 2) % 100 + 3;
      die_index = (die_index + 3) % 100;
      state.p1.position = (state.p1.position + die_roll - 1) % 10 + 1;
      state.p1.score += state.p1.position;
      if state.p1.score >= 1000 {
        return die_rolls * state.p2.score;
      }

      die_rolls += 3;
      let die_roll = die_index + (die_index + 1) % 100 + (die_index + 2) % 100 + 3;
      die_index = (die_index + 3) % 100;
      state.p2.position = (state.p2.position + die_roll - 1) % 10 + 1;
      state.p2.score += state.p2.position;
      if state.p2.score >= 1000 {
        return state.p1.score * die_rolls;
      }
    }
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut memo = HashMap::new();
    let (p1, p2) = quantum_dice(*input, &mut memo);
    u64::max(p1, p2)
  }
}
