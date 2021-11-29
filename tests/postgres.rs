#![cfg(feature = "postgres")]

use flp_gsp::{interpreter::postgres::*, parse};

#[test]
fn test_postgres() {
    let s = "((((! `age` -) & (! `age` > `18`)) & (`sex` ? [male, Male] | `sex` ~ `Female`)) & `name` * `J?c*`)";
    let search = parse(s).unwrap();

    let mut renames = PostgresRenames::new();
    renames.insert("name".into(), "t.name".into());
    renames.insert("sex".into(), "gender".into());

    let mut types = PostgresTypes::new();
    types.insert("age".into(), PostgresType::Int(None));

    let interpreted = interpret(&search, &renames, &types, 1).unwrap();
    let (clause, binds) = interpreted.get(0).unwrap();

    assert_eq!(
        clause,
        "((((NOT age IS NULL) AND (NOT age > $1)) AND (gender IN ($2, $3) OR gender ILIKE $4)) AND t.name ILIKE $5)"
    );
    assert_eq!(
        binds,
        &vec![
            PostgresType::Int(Some(18)),
            PostgresType::StringLike(Some("male".into())),
            PostgresType::StringLike(Some("Male".into())),
            PostgresType::StringLike(Some("Female".into())),
            PostgresType::StringLike(Some("J_c%".into()))
        ]
    );
}
