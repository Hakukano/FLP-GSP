#![cfg(feature = "hasura")]

use flp_gsp::{interpreter::hasura::*, Expression};

#[test]
fn test_sqlite() {
    let s = r#"(  (((! "age" -) & (! "age" > "18")) & ("sex" ? ["male", "Male"     ] | "sex" ~ "Female")) & "name" * "J?c*")"#;
    let expression = Expression::try_from_str(s).unwrap();

    let mut types = HasuraTypes::new();
    types.insert("age".into(), HasuraType::Integer);
    types.insert("sex".into(), HasuraType::StringLike);
    types.insert("name".into(), HasuraType::StringLike);

    let interpreted = interpret(&expression, &types).unwrap();
    let clause = interpreted;

    assert_eq!(
        clause,
        "{_and:[{_and:[{_and:[{_not:{age:{_is_null:true}}},{_not:{age:{_gt:18}}}]},{_or:[{sex:{_in:[\"male\",\"Male\"]}},{sex:{_ilike:\"Female\"}}]}]},{name:{_ilike:\"J_c%\"}}]}"
    );
}
