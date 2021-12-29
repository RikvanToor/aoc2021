use crate::days::Day;
use crate::parser::grid;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map as pmap;
use nom::multi::many0;
use nom::IResult;
use pathfinding::directed::dijkstra::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amphipod {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
}

use Amphipod::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
  Pod(Amphipod),
  Open,
  Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State<const DIM: usize> {
  rooms: [[Option<Amphipod>; DIM]; 4],
  hallway: [Option<Amphipod>; 12],
}

use Tile::*;

fn parse_tile(input: &str) -> IResult<&str, Tile> {
  alt((
    pmap(tag("A"), |_| Pod(A)),
    pmap(tag("B"), |_| Pod(B)),
    pmap(tag("C"), |_| Pod(C)),
    pmap(tag("D"), |_| Pod(D)),
    pmap(tag("#"), |_| Wall),
    pmap(tag(" "), |_| Wall),
    pmap(tag("."), |_| Open),
  ))(input)
}

fn init_state<const DIM: usize>(input: &[Vec<Tile>]) -> State<DIM> {
  let mut state = State {
    rooms: [[None; DIM]; 4],
    hallway: [None; 12],
  };
  for (y, row) in input.iter().enumerate().skip(2).take(DIM) {
    if let Pod(p) = row[3] {
      state.rooms[0][y - 2] = Some(p);
    }
    if let Pod(p) = row[5] {
      state.rooms[1][y - 2] = Some(p);
    }
    if let Pod(p) = row[7] {
      state.rooms[2][y - 2] = Some(p);
    }
    if let Pod(p) = row[9] {
      state.rooms[3][y - 2] = Some(p);
    }
  }
  for (x, mp) in input[1].iter().enumerate().skip(1).take(11) {
    if let Pod(p) = mp {
      state.hallway[x] = Some(*p);
    }
  }

  state
}

const GOALS: [usize; 4] = [3, 5, 7, 9];
const COSTS: [usize; 4] = [1, 10, 100, 1000];
const VALID_XS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

fn get_successors2<const DIM: usize>(state: &State<DIM>) -> Vec<(State<DIM>, usize)> {
  let mut states = vec![];
  for (i, room) in state.rooms.iter().enumerate() {
    for j in 0..DIM {
      if let Some(p) = room[j] {
        let mut can_move = p as usize != i;
        if !can_move {
          for mp2 in room.iter().skip(j + 1) {
            if mp2.unwrap() as usize != i {
              can_move = true;
              break;
            }
          }
        }

        if can_move {
          let x_start = GOALS[i];
          for x in VALID_XS {
            let mut reachable = true;
            if x_start < x {
              for dx in x_start + 1..=x {
                if state.hallway[dx].is_some() {
                  reachable = false;
                  break;
                }
              }
            } else {
              for dx in x..x_start {
                if state.hallway[dx].is_some() {
                  reachable = false;
                  break;
                }
              }
            }

            if reachable {
              let steps = j
                + 1
                + if x > x_start {
                  x - x_start
                } else {
                  x_start - x
                };
              let cost = steps * COSTS[p as usize];
              let mut new_state = state.clone();
              new_state.hallway[x] = Some(p);
              new_state.rooms[i][j] = None;
              states.push((new_state, cost));
            }
          }
        }

        break;
      }
    }
  }

  for x_start in VALID_XS {
    if let Some(p) = state.hallway[x_start] {
      let x_goal = GOALS[p as usize];
      if state.rooms[p as usize].iter().any(|mp| match mp {
        None => false,
        Some(p2) => p2 != &p,
      }) {
        continue;
      }
      if state.rooms[p as usize][0].is_some() {
        continue;
      }

      let mut y_goal = 0;
      for y in 1..DIM {
        if state.rooms[p as usize][y].is_none() {
          y_goal = y;
        }
      }

      let mut reachable = true;

      if x_start < x_goal {
        for dx in x_start + 1..=x_goal {
          if state.hallway[dx].is_some() {
            reachable = false;
            break;
          }
        }
      } else {
        for dx in x_goal..x_start {
          if state.hallway[dx].is_some() {
            reachable = false;
            break;
          }
        }
      }

      if reachable {
        let steps = y_goal
          + 1
          + if x_goal > x_start {
            x_goal - x_start
          } else {
            x_start - x_goal
          };
        let cost = steps * COSTS[p as usize];
        let mut new_state = state.clone();
        new_state.rooms[p as usize][y_goal] = Some(p);
        new_state.hallway[x_start] = None;
        states.push((new_state, cost));
      }
    }
  }

  states
}

fn success2<const DIM: usize>(state: &State<DIM>) -> bool {
  state.rooms[0].iter().all(|mp| mp == &Some(A))
    && state.rooms[1].iter().all(|mp| mp == &Some(B))
    && state.rooms[2].iter().all(|mp| mp == &Some(C))
    && state.rooms[3].iter().all(|mp| mp == &Some(D))
}

pub struct Day23;

impl Day for Day23 {
  type Input = Vec<Vec<Tile>>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    // Ok(("", String::from(input)))
    grid(parse_tile)(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    dijkstra(&init_state(input), |s| get_successors2::<2>(s), success2)
      .unwrap()
      .1
  }

  type Output2 = usize;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let mut input = input.clone();
    let (_, row_1) = many0(parse_tile)("  #D#C#B#A#").unwrap();
    let (_, row_2) = many0(parse_tile)("  #D#B#A#C#").unwrap();

    input.insert(3, row_1);
    input.insert(4, row_2);

    dijkstra(
      &init_state::<4>(&input),
      |s| get_successors2::<4>(s),
      success2,
    )
    .unwrap()
    .1
  }
}
