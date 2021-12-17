use nom::bits::complete as bits;
use nom::branch::alt;
use nom::combinator::eof;
use nom::combinator::map as pmap;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::IResult;

use crate::days::Day;

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
    value: u64,
  },
  OperatorPacket {
    version: u8,
    operator: Operator,
    sub_packets: Vec<Packet>,
  },
}

use Operator::*;
use Packet::*;

fn parse_transmission(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  let (cont, packet) = parse_packet(input)?;
  // Consume optional zeroes at the end of input
  let (cont, _) = many0(bits::tag(0, 1usize))(cont)?;
  Ok((cont, packet))
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  // Take version
  let (cont, version) = bits::take(3usize)(input)?;
  // Parse either a literal value or an operator packet
  alt((parse_literal(version), parse_operator_packet(version)))(cont)
}

fn parse_literal(version: u8) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  move |input| {
    // A literal has type Id 4
    let (cont, _) = bits::tag(4, 3usize)(input)?;
    // Parse many 5-bit blocks starting with a 1
    let (cont, mut blocks) = many0(parse_literal_block)(cont)?;
    // Parse the final 5-bit block starting with a 0
    let (cont, _) = bits::tag(0, 1usize)(cont)?;
    let (cont, last_block) = bits::take(4usize)(cont)?;

    blocks.push(last_block);
    // Fold the blocks to a u64 value
    let res = Literal {
      version: version,
      value: blocks.iter().fold(0, |acc, x| (acc << 4) + *x as u64),
    };

    Ok((cont, res))
  }
}

fn parse_literal_block(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
  // Only continue when the first bit is a 1, return the 4 bits after
  let (cont, _) = bits::tag(1, 1usize)(input)?;
  bits::take(4usize)(cont)
}

fn parse_operator_packet(
  version: u8,
) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  move |input| {
    // Get the operator and continue
    let (cont, operator) = parse_operator(input)?;
    alt((
      parse_length_operator(version, operator),
      parse_count_operator(version, operator),
    ))(cont)
  }
}

fn parse_operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
  alt((
    pmap(bits::tag(0, 3usize), |_| Sum),
    pmap(bits::tag(1, 3usize), |_| Product),
    pmap(bits::tag(2, 3usize), |_| Min),
    pmap(bits::tag(3, 3usize), |_| Max),
    pmap(bits::tag(5, 3usize), |_| GreaterThan),
    pmap(bits::tag(6, 3usize), |_| LessThan),
    pmap(bits::tag(7, 3usize), |_| EqualTo),
  ))(input)
}

fn parse_length_operator(
  version: u8,
  operator: Operator,
) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  move |input| {
    // Length operators have a 0 after the operator type
    let (cont, _) = bits::tag(0, 1usize)(input)?;
    // The length is specified in the next 15 bits
    let (cont, length) = bits::take(15usize)(cont)?;
    // Parse sub-packages in the next X bits according to the length we just parsed
    // Only succeeds if the entire length is used
    let (cont, sub_packets) = parse_with_length(length, many0(parse_packet))(cont)?;
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
) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
  move |input| {
    // Count operators have a 1 after the operator type
    let (cont, _) = bits::tag(1, 1usize)(input)?;
    // The next 11 bits represent the number of sub_packages
    let (cont, count) = bits::take(11usize)(cont)?;
    // Parse exactly that many sub-packages
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
    Literal { value, .. } => *value as u64,
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

// Apply a parser to the first X bits of the input,
fn parse_with_length<'a, T, F>(
  count: usize,
  mut f: F,
) -> impl FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), T>
where
  F: FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), T>,
{
  move |(input, offset)| {
    // Calculate how many bytes the left and right side of the split will contain
    let total_size_left = offset + count;
    let left_segments = (total_size_left - 1) / 8 + 1;

    let input_segments = input.len();
    let total_size_right = input_segments * 8 - total_size_left;
    let right_segments = (total_size_right - 1) / 8 + 1;
    let right_offset = total_size_left % 8;

    // Split the input into two sides
    let left: &[u8] = &input[0..left_segments];
    let right: &[u8] = &input[input_segments - right_segments..input_segments];

    // Apply the parser to the left side
    let (cont_l, inner) = f((left, offset))?;
    // Ensure the complete left side has been consumed by the supplied parser
    let (cont_l, _): (_, u8) = if right_offset > 0 {
      bits::take(8 - right_offset)(cont_l)
    } else {
      Ok((cont_l, 0))
    }?;
    let (_, _) = eof(cont_l)?;

    // Return the result found in the left side, and continue with the right side
    Ok(((right, right_offset), inner))
  }
}

fn hex_to_bytes(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
  (0..input.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(&input[i..=i + 1], 16))
    .collect()
}

pub struct Day16;

impl Day for Day16 {
  type Input = Packet;

  fn parse(input: &str) -> IResult<&str, Self::Input> {
    let input_vec = hex_to_bytes(input).unwrap();
    let (_, package) = parse_transmission((&input_vec, 0)).unwrap();
    Ok(("", package))
  }

  type Output1 = usize;

  fn part_1(input: &Self::Input) -> Self::Output1 {
    sum_versions(&input)
  }

  type Output2 = u64;

  fn part_2(input: &Self::Input) -> Self::Output2 {
    eval(&input)
  }
}
