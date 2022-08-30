use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::map_res,
    sequence::tuple, IResult,
};

use super::{atom::*, comparison::*};

#[derive(Debug)]
pub enum Relation {
    C(Comparison),
    RAR {
        left: Box<Relation>,
        right: Box<Relation>,
    },
    RAC {
        left: Box<Relation>,
        right: Comparison,
    },
    CAR {
        left: Comparison,
        right: Box<Relation>,
    },
    CAC {
        left: Comparison,
        right: Comparison,
    },
    ROR {
        left: Box<Relation>,
        right: Box<Relation>,
    },
    ROC {
        left: Box<Relation>,
        right: Comparison,
    },
    COR {
        left: Comparison,
        right: Box<Relation>,
    },
    COC {
        left: Comparison,
        right: Comparison,
    },
    NR(Box<Relation>),
    NC(Comparison),
}

fn group_start(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}

fn group_end(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn c(input: &str) -> IResult<&str, Box<Relation>> {
    map_res(
        tuple((group_start, space0, comparison, space0, group_end)),
        |(_, _, c, _, _): (&str, &str, Comparison, &str, &str)| {
            Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(Relation::C(c)))
        },
    )(input)
}

macro_rules! bi_relation {
    ($fname:ident, $left_func:ident, $oper_func:ident, $right_func:ident, $left_type:ty, $oper_type:ident, $right_type:ty, $relation:ident) => {
        fn $fname(input: &str) -> IResult<&str, Box<Relation>> {
            map_res(
                tuple((
                    group_start,
                    space0,
                    $left_func,
                    space0,
                    $oper_func,
                    space0,
                    $right_func,
                    space0,
                    group_end,
                )),
                |(_, _, left, _, _, _, right, _, _): (
                    &str,
                    &str,
                    $left_type,
                    &str,
                    $oper_type,
                    &str,
                    $right_type,
                    &str,
                    &str,
                )| {
                    Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(
                        Relation::$relation { left, right },
                    ))
                },
            )(input)
        }
    };
}

bi_relation!(
    rar,
    relation,
    and,
    relation,
    Box<Relation>,
    And,
    Box<Relation>,
    RAR
);
bi_relation!(
    rac,
    relation,
    and,
    comparison,
    Box<Relation>,
    And,
    Comparison,
    RAC
);
bi_relation!(
    car,
    comparison,
    and,
    relation,
    Comparison,
    And,
    Box<Relation>,
    CAR
);
bi_relation!(cac, comparison, and, comparison, Comparison, And, Comparison, CAC);
bi_relation!(
    ror,
    relation,
    or,
    relation,
    Box<Relation>,
    Or,
    Box<Relation>,
    ROR
);
bi_relation!(
    roc,
    relation,
    or,
    comparison,
    Box<Relation>,
    Or,
    Comparison,
    ROC
);
bi_relation!(
    cor,
    comparison,
    or,
    relation,
    Comparison,
    Or,
    Box<Relation>,
    COR
);
bi_relation!(coc, comparison, or, comparison, Comparison, Or, Comparison, COC);

macro_rules! uni_relation {
    ($fname:ident, $oper_func:ident, $target_func:ident, $oper_type:ty, $target_type:ty, $relation:ident) => {
        fn $fname(input: &str) -> IResult<&str, Box<Relation>> {
            map_res(
                tuple((
                    group_start,
                    space0,
                    $oper_func,
                    space0,
                    $target_func,
                    space0,
                    group_end,
                )),
                |(_, _, _, _, target, _, _): (
                    &str,
                    &str,
                    $oper_type,
                    &str,
                    $target_type,
                    &str,
                    &str,
                )| {
                    Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(Box::new(
                        Relation::$relation(target),
                    ))
                },
            )(input)
        }
    };
}

uni_relation!(nr, not, relation, Not, Box<Relation>, NR);
uni_relation!(nc, not, comparison, Not, Comparison, NC);

pub fn relation(input: &str) -> IResult<&str, Box<Relation>> {
    map_res(
        alt((c, rar, rac, car, cac, ror, roc, cor, coc, nr, nc)),
        |r: Box<Relation>| Result::<Box<Relation>, nom::Err<nom::error::Error<&str>>>::Ok(r),
    )(input)
}
