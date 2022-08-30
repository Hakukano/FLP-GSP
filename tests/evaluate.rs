#![cfg(feature = "evaluate")]

use flp_gsp::{interpreter::evaluate::*, Expression};

mod common;

use common::*;

#[test]
fn test_evaluate() {
    let s = r#"(((! "age" > "18") & ("sex" ? ["male", "Male"] | "sex" ~ "Female")) & "name" * "J?c*")"#;
    let expression = Expression::try_from_str(s).unwrap();

    let mut rules = EvaluateRules::new();
    rules.insert("name".into(), EvaluateRule::default());
    rules.insert("age".into(), {
        let mut rule = EvaluateRule::default();
        rule.is_greater_than =
            |value, target| value.parse::<u8>().unwrap() > target.parse::<u8>().unwrap();
        rule.is_less_than =
            |value, target| value.parse::<u8>().unwrap() < target.parse::<u8>().unwrap();
        rule
    });
    rules.insert("sex".into(), EvaluateRule::default());

    let persons = vec![
        Person {
            name: "JacKkkk".into(),
            age: 18,
            sex: Sex::Male,
        },
        Person {
            name: "Joc".into(),
            age: 1,
            sex: Sex::Female,
        },
        Person {
            name: "jacKkkk".into(),
            age: 18,
            sex: Sex::Male,
        },
        Person {
            name: "JacKkkkew".into(),
            age: 20,
            sex: Sex::Male,
        },
        Person {
            name: "Jac".into(),
            age: 5,
            sex: Sex::Other,
        },
    ];

    let names = persons
        .into_iter()
        .filter_map(|a| {
            let mut pairs = EvaluatePairs::new();
            pairs.insert("name".into(), a.name.clone());
            pairs.insert("age".into(), a.age.to_string());
            pairs.insert("sex".into(), a.sex.into());

            if interpret(&expression, &rules, &pairs) {
                Some(a.name)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(names, vec!["JacKkkk", "Joc"]);
}

#[test]
fn test_invalid() {
    let s = r#"("="")"#;
    let expression = Expression::try_from_str(s);
    assert!(expression.is_err());
}
