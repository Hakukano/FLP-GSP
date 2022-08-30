#![cfg(feature = "mysql")]

use flp_gsp::{interpreter::mysql::*, Expression};

#[test]
fn test_mysql() {
    let s = r#"((((! "age" -) & (! "age" > "18")) & ("sex" ? ["male", "Male"] | "sex" ~ "Female")) & "name" * "J?c*")"#;
    let expression = Expression::try_from_str(s).unwrap();

    let mut renames = MysqlRenames::new();
    renames.insert("name".into(), "t.name".into());
    renames.insert("sex".into(), "gender".into());

    let mut types = MysqlTypes::new();
    types.insert("age".into(), MysqlType::Unsigned(None));
    types.insert("sex".into(), MysqlType::StringLike(None));
    types.insert("name".into(), MysqlType::StringLike(None));

    let interpreted = interpret(&expression, &renames, &types).unwrap();
    let (clause, binds) = interpreted;

    assert_eq!(
        clause,
        "((((NOT age IS NULL) AND (NOT age > ?)) AND (gender IN (?, ?) OR gender LIKE ?)) AND t.name LIKE ?)"
    );
    assert_eq!(
        binds,
        vec![
            MysqlType::Unsigned(Some(18)),
            MysqlType::StringLike(Some("male".into())),
            MysqlType::StringLike(Some("Male".into())),
            MysqlType::StringLike(Some("Female".into())),
            MysqlType::StringLike(Some("J_c%".into()))
        ]
    );
}
