use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::combinator::eof;
use nom::combinator::map as pmap;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;

use crate::days::Day;

fn parse_binary(input: &str) -> IResult<&str, Vec<u8>> {
  let mut binary_vec: Vec<u8> = vec![];
  for c in input.chars() {
    match c {
      '0' => binary_vec.extend_from_slice(&[0, 0, 0, 0]),
      '1' => binary_vec.extend_from_slice(&[0, 0, 0, 1]),
      '2' => binary_vec.extend_from_slice(&[0, 0, 1, 0]),
      '3' => binary_vec.extend_from_slice(&[0, 0, 1, 1]),
      '4' => binary_vec.extend_from_slice(&[0, 1, 0, 0]),
      '5' => binary_vec.extend_from_slice(&[0, 1, 0, 1]),
      '6' => binary_vec.extend_from_slice(&[0, 1, 1, 0]),
      '7' => binary_vec.extend_from_slice(&[0, 1, 1, 1]),
      '8' => binary_vec.extend_from_slice(&[1, 0, 0, 0]),
      '9' => binary_vec.extend_from_slice(&[1, 0, 0, 1]),
      'A' => binary_vec.extend_from_slice(&[1, 0, 1, 0]),
      'B' => binary_vec.extend_from_slice(&[1, 0, 1, 1]),
      'C' => binary_vec.extend_from_slice(&[1, 1, 0, 0]),
      'D' => binary_vec.extend_from_slice(&[1, 1, 0, 1]),
      'E' => binary_vec.extend_from_slice(&[1, 1, 1, 0]),
      'F' => binary_vec.extend_from_slice(&[1, 1, 1, 1]),
      _ => panic!("Not a hex string"), //TODO properly handle
    }
  }
  Ok(("", binary_vec))
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
  Sum,
  Product,
  Min,
  Max,
  GreaterThan,
  LessThan,
  EqualTo,
}

#[derive(Debug)]
pub enum Packet {
  Literal {
    version: u8,
    value: Vec<u8>,
  },
  OperatorPacket {
    version: u8,
    operator: Operator,
    sub_packets: Vec<Packet>,
  },
}

use Operator::*;
use Packet::*;

fn vec_to_nr(vec: &[u8]) -> u64 {
  vec.iter().fold(0, |acc, x| acc * 2 + *x as u64)
}

fn parse_transmission(input: &[u8]) -> IResult<&[u8], Packet> {
  let (cont, packet) = parse_packet(input)?;
  let (cont, _zeroes) = many0(tag(&[0]))(cont)?;
  Ok((cont, packet))
}

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
  let (cont, version_vec) = take(3usize)(input)?;
  let version = vec_to_nr(version_vec) as u8;
  alt((parse_literal(version), parse_operator_packet(version)))(cont)
}

fn parse_literal(version: u8) -> impl FnMut(&[u8]) -> IResult<&[u8], Packet> {
  move |input| {
    let (cont, _) = tag(&[1, 0, 0])(input)?;
    let (cont, mut first_blocks) = many0(parse_literal_block)(cont)?;
    let (cont, _) = tag(&[0])(cont)?;
    let (cont, last_block) = take(4usize)(cont)?;

    first_blocks.push(last_block.to_vec());
    let total_block = first_blocks.concat();
    let res = Literal {
      version: version,
      value: total_block,
    };
    Ok((cont, res))
  }
}

fn parse_literal_block(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
  let (cont, _) = tag(&[1])(input)?;
  let (cont, val) = take(4usize)(cont)?;
  Ok((cont, val.to_vec()))
}

fn parse_operator(input: &[u8]) -> IResult<&[u8], Operator> {
  alt((
    pmap(tag(&[0, 0, 0]), |_| Sum),
    pmap(tag(&[0, 0, 1]), |_| Product),
    pmap(tag(&[0, 1, 0]), |_| Min),
    pmap(tag(&[0, 1, 1]), |_| Max),
    pmap(tag(&[1, 0, 1]), |_| GreaterThan),
    pmap(tag(&[1, 1, 0]), |_| LessThan),
    pmap(tag(&[1, 1, 1]), |_| EqualTo),
  ))(input)
}

fn parse_operator_packet(version: u8) -> impl FnMut(&[u8]) -> IResult<&[u8], Packet> {
  move |input| {
    let (cont, operator) = parse_operator(input)?;
    alt((
      parse_length_operator(version, operator),
      parse_count_operator(version, operator),
    ))(cont)
  }
}

fn parse_length_operator(
  version: u8,
  operator: Operator,
) -> impl FnMut(&[u8]) -> IResult<&[u8], Packet> {
  move |input| {
    let (cont, _) = tag(&[0])(input)?;
    let (cont, bits) = take(15usize)(cont)?;
    let length = vec_to_nr(bits);
    let (cont, next_bits) = take(length as usize)(cont)?;
    // TODO error handling for parsing sub-packets
    let (_, (sub_packets, _)) = tuple((many0(parse_packet), eof))(next_bits)?;
    Ok((
      cont,
      OperatorPacket {
        version,
        operator,
        sub_packets,
      },
    ))
  }
}

fn parse_count_operator(
  version: u8,
  operator: Operator,
) -> impl FnMut(&[u8]) -> IResult<&[u8], Packet> {
  move |input| {
    let (cont, _) = tag(&[1])(input)?;
    let (cont, bits) = take(11usize)(cont)?;
    let count = vec_to_nr(bits) as usize;

    let (cont, sub_packets) = many_m_n(count, count, parse_packet)(cont)?;
    Ok((
      cont,
      OperatorPacket {
        version,
        operator,
        sub_packets,
      },
    ))
  }
}

fn sum_versions(packet: &Packet) -> usize {
  match packet {
    Literal { version: v, .. } => *v as usize,
    OperatorPacket {
      version: v,
      sub_packets: sp,
      ..
    } => *v as usize + sp.iter().map(sum_versions).sum::<usize>(),
  }
}

fn eval(packet: &Packet) -> u64 {
  match packet {
    Literal { value, .. } => vec_to_nr(value) as u64,
    OperatorPacket {
      sub_packets,
      operator,
      ..
    } => {
      let sub_vals = sub_packets.iter().map(eval);
      let sub_vals_vec: Vec<u64> = sub_vals.clone().collect();
      match operator {
        Sum => sub_vals.sum::<u64>(),
        Product => sub_vals.product::<u64>(),
        Min => sub_vals.min().unwrap(),
        Max => sub_vals.max().unwrap(),
        GreaterThan => {
          if sub_vals_vec[0] > sub_vals_vec[1] {
            1
          } else {
            0
          }
        }
        LessThan => {
          if sub_vals_vec[0] < sub_vals_vec[1] {
            1
          } else {
            0
          }
        }
        EqualTo => {
          if sub_vals_vec[0] == sub_vals_vec[1] {
            1
          } else {
            0
          }
        }
      }
    }
  }
}

pub struct Day16;

impl Day for Day16 {
  type Input = Vec<u8>;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    parse_binary(input)
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    let (_, packet) = parse_transmission(input).unwrap();
    sum_versions(&packet)
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    let (_, packet) = parse_transmission(input).unwrap();
    eval(&packet)
  }
}
