use std::ops::Deref;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::{none_of, one_of, space0},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Text(pub String);
impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}
pub fn text(input: &str) -> IResult<&str, Text> {
    let esc = escaped(none_of("\\\""), '\\', one_of("\\\""));
    let esc_or_empty = alt((esc, tag("")));
    map_res(delimited(tag("\""), esc_or_empty, tag("\"")), |s: &str| {
        Result::<Text, nom::Err<nom::error::Error<&str>>>::Ok(Text(
            s.replace("\\\\", "\\").replace("\\\"", "\"").to_string(),
        ))
    })(input)
}

#[derive(Debug)]
pub struct Array(pub Vec<String>);
impl Deref for Array {
    type Target = [String];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}
pub fn array(input: &str) -> IResult<&str, Array> {
    let left = pair(tag("["), space0);
    let right = pair(space0, tag("]"));
    let separator = tuple((space0, tag(","), space0));
    map_res(
        delimited(left, separated_list0(separator, text), right),
        |texts: Vec<Text>| {
            Result::<Array, nom::Err<nom::error::Error<&str>>>::Ok(Array(
                texts.into_iter().map(|t| t.0).collect(),
            ))
        },
    )(input)
}

macro_rules! operator {
    ($sname:ident, $fname:ident, $symbol:literal) => {
        #[derive(Debug)]
        pub struct $sname;
        pub fn $fname(input: &str) -> IResult<&str, $sname> {
            map_res(tag($symbol), |_| {
                Result::<$sname, nom::Err<nom::error::Error<&str>>>::Ok($sname)
            })(input)
        }
    };
}

operator!(Equal, equal, "=");
operator!(EqualCI, equal_ci, "~");
operator!(Greater, greater, ">");
operator!(Less, less, "<");
operator!(Wildcard, wildcard, "*");
operator!(Regex, regex, "$");
operator!(Any, any, "?");
operator!(Null, null, "-");
operator!(And, and, "&");
operator!(Or, or, "|");
operator!(Not, not, "!");
