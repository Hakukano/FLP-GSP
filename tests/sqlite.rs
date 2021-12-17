#![cfg(feature = "sqlite")]

use flp_gsp::{interpreter::sqlite::*, parse};

#[test]
fn test_sqlite() {
    let s = "((((! `age` -) & (! `age` > `18`)) & (`sex` ? [male, Male] | `sex` ~ `Female`)) & `name` * `J?c*`)";
    let search = parse(s).unwrap();

    let mut renames = SqliteRenames::new();
    renames.insert("name".into(), "t.name".into());
    renames.insert("sex".into(), "gender".into());

    let mut types = SqliteTypes::new();
    types.insert("age".into(), SqliteType::Integer(None));
    types.insert("sex".into(), SqliteType::Text(None));
    types.insert("name".into(), SqliteType::Text(None));

    let interpreted = interpret(&search, &renames, &types).unwrap();
    let (clause, binds) = interpreted.get(0).unwrap();

    assert_eq!(
        clause,
        "((((NOT age IS NULL) AND (NOT age > ?)) AND (gender IN (?, ?) OR gender LIKE ?)) AND t.name LIKE ?)"
    );
    assert_eq!(
        binds,
        &vec![
            SqliteType::Integer(Some(18)),
            SqliteType::Text(Some("male".into())),
            SqliteType::Text(Some("Male".into())),
            SqliteType::Text(Some("Female".into())),
            SqliteType::Text(Some("J_c%".into()))
        ]
    );
}
