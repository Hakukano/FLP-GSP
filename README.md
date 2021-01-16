# FLP-GSP
General Search Parser

# LALR(1) Grammer

## Search:

* Statements

## Statements:

* []
* Statements Relation

## Relation:

* GroupStart Comparison GroupEnd
* GroupStart Relation And Relation GroupEnd
* GroupStart Relation And Comparison GroupEnd
* GroupStart Comparison And Relation GroupEnd
* GroupStart Comparison And Comparison GroupEnd
* GroupStart Relation Or Relation GroupEnd
* GroupStart Relation Or Comparison GroupEnd
* GroupStart Comparison Or Relation GroupEnd
* GroupStart Comparison Or Comparison GroupEnd
* GroupStart Not Relation GroupEnd
* GroupStart Not Comparison GroupEnd

## Comparison

* Str Equal Str
* Str EqualCI Str
* Str Greater Str
* Str Less Str
* Str Wildcard Str
* Str Regex Str

## Str

* \`\[^\`\]\*\`

## GroupStart

* (

## GroupEnd

* )

## And

* &

## Or

* |

## Not

* !

## Equal

* =

## EqualCI

* ~

## Greater

* \>

## Less

* <

## Wildcard

* \*

## Regex

* $

# Interpreter

Some example interpreters that maybe useful

## Evaluate

Customizable in-code evaluating interpreter. [Goto](https://github.com/Hakukano/FLP-GSP/blob/main/src/interpreter/evaluate.rs)

An example could be found [here](https://github.com/Hakukano/FLP-GSP/blob/main/tests/evaluate.rs)

### Concept

To evaluate a `Search`, you will need `EvaluateRules` and `EvaluatePairs`

* `EvaluateRules`: You can overwrite any default rules for comparison. Usually, you may want to overwrite `is_greater_than` and `is_less_than` if the key has a numeric value. Rules should be reused as often as possible in order to reduce redundant codes.

* `EvaluatePairs`: Actual key-value pairs for the evaluation. You need to parse your values into strings so that rules can be applied.
