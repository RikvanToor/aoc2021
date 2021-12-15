use nom::character::complete::{newline, satisfy};
use nom::error::{Error, ParseError};
use nom::multi::{many1, separated_list1};
use nom::{AsChar, Err, IResult, InputIter, InputLength, Parser, Slice};
use std::ops::RangeFrom;

pub fn digit<T: From<u32>>(input: &str) -> IResult<&str, T> {
  let (cont, c) = satisfy(|c| c.is_digit(10))(input)?;
  let res = c.to_digit(10).unwrap();
  Ok((cont, T::from(res)))
}

pub fn digit_grid<T: From<u32>>(input: &str) -> IResult<&str, Vec<Vec<T>>> {
  grid(digit)(input)
}

pub fn grid<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, Vec<Vec<O>>, E>
where
  I: Slice<RangeFrom<usize>> + InputIter + InputLength + Clone,
  <I as InputIter>::Item: AsChar,
  F: Parser<I, O, E>,
  E: ParseError<I>,
{
  separated_list1(newline, many1(f))
}

#[derive(Debug)]
pub enum MyErr {
  FileError(std::io::Error),
  ParseError(Err<Error<String>>),
}

impl From<Err<Error<&str>>> for MyErr {
  fn from(e: Err<Error<&str>>) -> MyErr {
    let inner_err = match e {
      Err::Incomplete(n) => Err::Incomplete(n),
      Err::Error(e) => Err::Error(conv_error(e)),
      Err::Failure(e) => Err::Failure(conv_error(e)),
    };
    MyErr::ParseError(inner_err)
  }
}

impl From<std::io::Error> for MyErr {
  fn from(e: std::io::Error) -> MyErr {
    MyErr::FileError(e)
  }
}

fn conv_error(e: Error<&str>) -> Error<String> {
  Error {
    input: e.input.to_owned(),
    code: e.code,
  }
}
