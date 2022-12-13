use nom::{bytes::complete::take_while, IResult};

pub fn ws0(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}
