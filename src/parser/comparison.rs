use nom::{character::complete::space0, combinator::map_res, sequence::tuple, IResult};

use super::atom::*;

macro_rules! bi_comparison {
    ($sname:ident, $left_type:ty, $oper_type:ty, $right_type:ty, $fname:ident, $left_func:ident, $oper_func:ident, $right_func:ident) => {
        #[derive(Debug)]
        pub struct $sname {
            pub left: $left_type,
            pub right: $right_type,
        }
        pub fn $fname(input: &str) -> IResult<&str, $sname> {
            map_res(
                tuple(($left_func, space0, $oper_func, space0, $right_func)),
                |(left, _, _, _, right): ($left_type, &str, $oper_type, &str, $right_type)| {
                    Result::<$sname, nom::Err<nom::error::Error<&str>>>::Ok($sname { left, right })
                },
            )(input)
        }
    };
}

bi_comparison!(IsEqual, Text, Equal, Text, is_equal, text, equal, text);
bi_comparison!(
    IsEqualCI,
    Text,
    EqualCI,
    Text,
    is_equal_ci,
    text,
    equal_ci,
    text
);
bi_comparison!(IsGreater, Text, Greater, Text, is_greater, text, greater, text);
bi_comparison!(IsLess, Text, Less, Text, is_less, text, less, text);
bi_comparison!(
    IsWildcard,
    Text,
    Wildcard,
    Text,
    is_wildcard,
    text,
    wildcard,
    text
);
bi_comparison!(IsRegex, Text, Regex, Text, is_regex, text, regex, text);
bi_comparison!(IsAny, Text, Any, Array, is_any, text, any, array);

macro_rules! uni_comparison {
    ($sname:ident, $oper_type:ident, $target_type:ident, $fname:ident, $oper_func:ident, $target_func:ident) => {
        #[derive(Debug)]
        pub struct $sname(pub $target_type);
        pub fn $fname(input: &str) -> IResult<&str, $sname> {
            map_res(
                tuple(($target_func, space0, $oper_func)),
                |(target, _, _): ($target_type, &str, $oper_type)| {
                    Result::<$sname, nom::Err<nom::error::Error<&str>>>::Ok($sname(target))
                },
            )(input)
        }
    };
}

uni_comparison!(IsNull, Null, Text, is_null, null, text);

macro_rules! comparison {
    ($(($sname:ident, $fname:ident),)*) => {
        #[derive(Debug)]
        pub enum Comparison {
            $(
            $sname($sname),
            )*
        }
        pub fn comparison(input: &str) -> IResult<&str, Comparison> {
            $(
            if let Ok((rest, matched)) = $fname(input) {
                return Ok((rest, Comparison::$sname(matched)));
            }
            )*
            Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))
        }
    };
}

comparison!(
    (IsEqual, is_equal),
    (IsEqualCI, is_equal_ci),
    (IsGreater, is_greater),
    (IsLess, is_less),
    (IsWildcard, is_wildcard),
    (IsRegex, is_regex),
    (IsAny, is_any),
    (IsNull, is_null),
);
