use flp_gsp::{interpreter::mysql::*, parse};

#[test]
fn test_mysql() {
    let s = "(((! `age` > `18`) & (`sex` ~ `male` | `sex` ~ `Female`)) & `name` * `J?c*`)";
    let search = parse(s, false).unwrap();
    println!("{:?}", search);

    let mut renames = MysqlRenames::new();
    renames.insert("name".into(), "t.name".into());
    renames.insert("sex".into(), "gender".into());

    let mut types = MysqlTypes::new();
    types.insert("age".into(), MysqlType::Unsigned("".into()));

    let interpreted = mysql(&search, &renames, &types);
    let (clause, binds) = interpreted.get(0).unwrap();

    assert_eq!(
        clause,
        "(((NOT `age` > ?) AND (`gender` = ? OR `gender` = ?)) AND `t.name` LIKE ?)"
    );
    assert_eq!(
        binds,
        &vec![
            MysqlType::Unsigned("18".into()),
            MysqlType::StringLike("male".into()),
            MysqlType::StringLike("Female".into()),
            MysqlType::StringLike("J_c%".into())
        ]
    );
}
