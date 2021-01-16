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
