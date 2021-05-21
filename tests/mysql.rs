use flp_gsp::{interpreter::mysql::*, parse};

#[test]
fn test_mysql() {
    let s = "((((! `age` -) & (! `age` > `18`)) & (`sex` ? [male, Male] | `sex` ~ `Female`)) & `name` * `J?c*`)";
    let search = parse(s).unwrap();
    println!("{:?}", search);

    let mut renames = MysqlRenames::new();
    renames.insert("name".into(), "t.name".into());
    renames.insert("sex".into(), "gender".into());

    let mut types = MysqlTypes::new();
    types.insert("age".into(), MysqlType::Unsigned(None));

    let interpreted = interpret(&search, &renames, &types).unwrap();
    let (clause, binds) = interpreted.get(0).unwrap();

    assert_eq!(
        clause,
        "((((NOT `age` IS NULL) AND (NOT `age` > ?)) AND (`gender` IN (?, ?) OR `gender` LIKE ?)) AND `t.name` LIKE ?)"
    );
    assert_eq!(
        binds,
        &vec![
            MysqlType::Unsigned(Some(18)),
            MysqlType::StringLike(Some("male".into())),
            MysqlType::StringLike(Some("Male".into())),
            MysqlType::StringLike(Some("Female".into())),
            MysqlType::StringLike(Some("J_c%".into()))
        ]
    );
}
