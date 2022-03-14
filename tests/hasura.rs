#![cfg(feature = "hasura")]

use flp_gsp::{interpreter::hasura::*, parse};

#[test]
fn test_sqlite() {
    let s = "((((! `age` -) & (! `age` > `18`)) & (`sex` ? [male, Male] | `sex` ~ `Female`)) & `name` * `J?c*`)";
    let search = parse(s).unwrap();

    let mut types = HasuraTypes::new();
    types.insert("age".into(), HasuraType::Integer);
    types.insert("sex".into(), HasuraType::StringLike);
    types.insert("name".into(), HasuraType::StringLike);

    let interpreted = interpret(&search, &types).unwrap();
    let clause = interpreted.get(0).unwrap();

    assert_eq!(
        clause,
        "{_and:[{_and:[{_and:[{_not:{age:{_is_null:true}}},{_not:{age:{_gt:18}}}]},{_or:[{sex:{_in:[\"male\",\"Male\"]}},{sex:{_ilike:\"Female\"}}]}]},{name:{_ilike:\"J_c%\"}}]}"
    );
}
