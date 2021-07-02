# computor_v1
📚  Simple Equation Solver Program

## Description

This program solves polynomial equations from 0 to 2 degrees.

## Usage

If you don't have Rust installed:

```
docker build -t computor_v1 .
docker run -ti computor_v1
```

Locally or in your container:
`cargo run <equation>`

The `equation` has to follow a simple format like:
`7 * X^0 + 14 * X^1 - 6 * X^2 = 13 * X ^ 0 - 1 * X ^ 1 + 2 * X ^ 2`

But you also can write it in a more natural way:
`7 + 14X -6X^2 = 13 - X + 2X^2`

## Walkthrough

The lexing and parsing parts of this program use compilation theory inspired tools.

They particularly rely on a French [paper](https://henri-garreta.developpez.com/tutoriels/techniques-outils-compilation/) written by Henri Garreta.

### Lexer

The first part of the program aims at recognizing at tokens (ie: minimal language units, or lexems) like a `Number` or `=`.

For this, a [finite-state machine](https://en.wikipedia.org/wiki/Finite-state_machine) is used, with a transition table as you can see below:

| |  | whitespace | "+" | "-" | "*" | "^" | "=" | "X" | numeric | "." | other | "\0" |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| | | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| Initial |	0 | 0 | 1 | 2 | 3 | 4 |5 | 6 | 7 | 13 | 13 | 12 |
| Final (`Plus`) | 1 | 1	| 1	| 1	| 1	| 1	| 1	| 1	| 1	| 1	| 1	| 1 |
| Final (`Minus`) | 2	| 2 | 2	| 2	| 2	| 2	| 2	| 2	| 2	| 2	| 2	| 2 |
| Final (`Mult`) | 3	| 3	| 3	| 3	| 3	| 3	| 3	| 3	| 3	| 3	| 3	| 3 |
| Final (`Power`) | 4 | 4	| 4	| 4	| 4	| 4	| 4 | 4	| 4	| 4 | 4	| 4 |
| Final (`Equal`) | 5	| 5	| 5	| 5	| 5	| 5	| 5	| 5	| 5	| 5	| 5	| 5 |
| Final (`X`) | 6	| 6	| 6	| 6	| 6	| 6	| 6	| 6	| 6	| 6	| 6	| 6 |
| Transitory | 7 | 11 |	11 | 11 | 11 | 11 | 11 | 11 | 7 | 8 | 11 | 11 |
| Transitory | 8 | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 9 | 13 | 13 | 13 |
| Transitory | 9 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 9 | 10 | 10 | 10 |
| FinalStar(`Number`) | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 | 10 |
| FinalStar(`Number`) | 11 | 11 | 11 | 11 | 11 | 11 | 11 | 11 | 11 | 11 | 11 | 11 |
| Final (`End`) | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12 | 12	| 12 | 12 |
| Error | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 13 | 13 |

Concretely depending on the state we are and the character we are currently reading, we will make a transition to another state that could be:
* an initial one: generally when the state machine starts consuming, or when it consumes whitespaces
* a transitory one: when it is reading characters as parts of a token
* a final one: when the last character of a token has been consumed
* a final star one: when the state machine has had to read an extra character to detect a token
* an error one: when an unexpected character is consumed

### Parser

The coded parser is the result of a [context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar) `G = (VT , VN, S0, P)` with:
* `VT`: a set of terminal symbols (ie our tokens): `Plus`, `Minus`, `Mult`, `Power`, `Equal`, `X`, `Number`, `End`
* `VN`: a set of non terminal symbols that can be derived in a combination of other `VN` and / or `VT` (see the production part below)
* `S0`: a particular `VN`, as it is the start symbol axiom
* `P`: a set of productions of type allowing to derive the `VN` (`VN` capital letters and `VT` in camel case (`X` is a `VT`)) :
	```
	EQUATION -> EXPRESSION Equal EXPRESSION End
	EXPRESSION -> Plus TERM EXPRESSION_END | Minus TERM EXPRESSION_END | TERM EXPRESSION_END
	EXPRESSION_END -> Plus TERM EXPRESSION_END | Minus TERM EXPRESSION_END | ε (= none of the two)
	TERM -> Number TERM_END | X Degree
	TERM_END -> Mult X DEGREE | X DEGREE | ε
	DEGREE -> Power Number | ε
	```
Those previous rules are followed using a recursive descent analysis.
