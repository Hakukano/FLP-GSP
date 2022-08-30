# FLP-GSP

[![Crates.io Version](https://img.shields.io/crates/v/flp-gsp.svg)](https://crates.io/crates/flp-gsp)

General Search Parser

# General Search String (GSS) Builder

[Goto the page](https://hakukano.github.io).

# General Search String (GSS) LALR(1) Grammar

```
    Search -> Relation

  Relation -> GroupStart Comparison GroupEnd
           -> GroupStart Relation And Relation GroupEnd
           -> GroupStart Relation And Comparison GroupEnd
           -> GroupStart Comparison And Relation GroupEnd
           -> GroupStart Comparison And Comparison GroupEnd
           -> GroupStart Relation Or Relation GroupEnd
           -> GroupStart Relation Or Comparison GroupEnd
           -> GroupStart Comparison Or Relation GroupEnd
           -> GroupStart Comparison Or Comparison GroupEnd
           -> GroupStart Not Relation GroupEnd
           -> GroupStart Not Comparison GroupEnd

Comparison -> Str Equal Str
           -> Str EqualCI Str
           -> Str Greater Str
           -> Str Less Str
           -> Str Wildcard Str
           -> Str Regex Str
           -> Str Any Array
           -> Str Null

       Str -> DoubleQuote Content DoubleQuote
       
     Array -> SquareBracketLeft (Str Comma)* SquareBracketRight

GroupStart -> (

  GroupEnd -> )

       And -> &

        Or -> |

       Not -> !

     Equal -> =

   EqualCI -> ~

   Greater -> >

      Less -> <

  Wildcard -> *

     Regex -> $
     
       Any -> ?

      Null -> -
```

# Interpreter

Some example interpreters that maybe useful, need to be enabled by feature

## Evaluate ["evaluate"]

Customizable in-code evaluating interpreter. [Goto the file](https://github.com/Hakukano/FLP-GSP/blob/main/src/interpreter/evaluate.rs).

An example could be found [here](https://github.com/Hakukano/FLP-GSP/blob/main/tests/evaluate.rs).

### Concept

To evaluate a `Search`, you will need `EvaluateRules` and `EvaluatePairs`.

* `EvaluateRules`: You can overwrite any default rules for comparison. Usually, you may want to overwrite `is_greater_than` and `is_less_than` if the key has a numeric value. Rules should be reused as often as possible in order to reduce redundant codes.

* `EvaluatePairs`: Actual key-value pairs for the evaluation. You need to parse your values into strings so that rules can be applied.

## Mysql ["mysql"]

Generating Mysql condition clause. [Goto the file](https://github.com/Hakukano/FLP-GSP/blob/main/src/interpreter/mysql.rs).

An example could be found [here](https://github.com/Hakukano/FLP-GSP/blob/main/tests/mysql.rs).

The types can be used in [sqlx^0.5](https://crates.io/crates/sqlx) binding directly.

### Concept

To generate Mysql condition clause from a `Search`, you will need `MysqlRenames` and `MysqlTypes`.

* `MysqlRenames`: You can insert any rename rules to it. E.g. key `sex` in search string may need to be renamed to `table_a.gender` regarding the actual query string. All keys without rename rules will stay as is.

* `MysqlTypes`: You can insert any types to it. You need this because the condition clause is a prepare clause (i.e. all values are replaced as placeholder(?)) and you will be given a Vec<MysqlType> with search targets in the order that "?"s appear in the clause. For details, please read the example. Additionally, MysqlType::StringLike(String) is the default type if you didn't insert types for one or some keys.

## Postgres ["postgres"]

Very similar to Mysql except for some types.

The types can be used in [sqlx^0.5](https://crates.io/crates/sqlx) binding directly, except several types defined by sqlx itself, e.g. `PgInterval`, `PgMoney`, etc.

### Special Types

* VarBit: `target` need to be in format of `<u64>`. E.g. `1024` stands for bits `0000010000000000`
