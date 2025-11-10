# Enums

## else if vs match

* Prefer `match` over `else if` when working with a single variable.
* `match` considers all possibilities and creates more robust code  (i.e. there is greater confidence that your code is error free).
* Use an underscore as a match "catch all" (similar to `else` clauses).
* When `match` is used with enums, your programs can be more robust because they catch every possible scenario.

## Expressions

* Rust is an expression-based language, which means that you can set a variable to equal the result of a condition evaluation (e.g. with an `if else` or `match` statement). 
* This is similar to JavaScript ternary expressions. 
