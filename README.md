# LispDM

## Description

LispDM is a Scheme interpreter written in Rust. It is based on the R7RS standard and provides a minimal set of built-in functions to write programs in Scheme. The interpreter is implemented as a library and can be used in other projects.

[Docs](https://abbit.github.io/lisp-machine)

## Features

- Tail call optimization for recursion
- Macros (like Clojure's `defmacro`)
- Input-output (console and file)
- Lazy evaluation
- Pattern matching
- REPL
- Standard library

## R7RS compatibility

### Data types

- [x] `boolean` (#t, #f)
- [x] `number` (integer, real)
- [ ] `number` (rational, complex)
- [x] `string`
- [x] `symbol`
- [x] `list`
- [x] `pair` (`improper list`)
- [x] `char`
- [x] `procedure`
- [x] `port`
- [ ] `vector`
- [ ] `bytevector`

### Special forms

Implemented in Rust:

- [x] `define`
- [x] `define-macro`
- [x] `set!`
- [x] `lambda`
- [x] `let` (and named let)
- [x] `letrec`
- [x] `if`
- [x] `cond`
- [x] `do`
- [x] `quote`
- [x] `quasiquote`
- [x] `unquote`
- [x] `unquote-splicing`
- [x] `include`
- [x] `load`

Implemented in Scheme:

- [x] `let*`
- [x] `letrec*`
- [x] `case`
- [x] `when`
- [x] `unless`
- [x] `and`
- [x] `or`

### Standard library

- equivalence predicates:
  - [x] `eqv?`
  - [x] `eq?`
  - [x] `equal?`
- numbers:
  - [x] `number?`
  - [x] `+`
  - [x] `-`
  - [x] `*`
  - [x] `/`
  - [x] `=`
  - [x] `<`
  - [x] `>`
  - [x] `<=`
  - [x] `>=`
  - [x] `integer?`
  - [x] `zero?`
  - [x] `positive?`
  - [x] `negative?`
  - [x] `odd?`
  - [x] `even?`
  - [x] `abs`
  - [x] `min`
  - [x] `max`
  - [x] `modulo`
  - [x] `quotient`
  - [x] `remainder`
  - [x] `floor`
  - [x] `ceiling`
  - [x] `truncate`
  - [x] `round`
  - [x] `square`
  - [x] `sqrt`
  - [x] `expt`
  - [x] `number->string`
  - [x] `string->number`
- booleans:
  - [x] `boolean?`
  - [x] `not`
  - [x] `and`
  - [x] `or`
- pairs and lists:
  - [x] `pair?`
  - [x] `cons`
  - [x] `car`
  - [x] `cdr`
  - [x] `caar`
  - [x] `cadr`
  - [x] `cdar`
  - [x] `cddr`
  - [x] `caaar`, `caadr`, ..., `cddar`, `cdddr`
  - [x] `null?`
  - [x] `list?`
  - [x] `make-list`
  - [x] `list`
  - [x] `length`
  - [x] `append`
  - [x] `reverse`
  - [x] `list-tail`
  - [x] `list-ref`
  - [x] `memq`
  - [x] `memv`
  - [x] `member`
  - [x] `assq`
  - [x] `assv`
  - [x] `assoc`
  - [x] `list-copy`
- symbols:
  - [x] `symbol?`
  - [x] `symbol->string`
  - [x] `string->symbol`
- characters:
  - [x] `char?`
  - [x] `char=?`
  - [x] `char<?`
  - [x] `char>?`
  - [x] `char<=?`
  - [x] `char>=?`
  - [x] `char-alphabetic?`
  - [x] `char-numeric?`
  - [x] `char-whitespace?`
  - [x] `char-upper-case?`
  - [x] `char-lower-case?`
  - [x] `digit-value`
  - [x] `char->integer`
  - [x] `integer->char`
  - [x] `char-upcase`
  - [x] `char-downcase`
  - [x] `char-foldcase`
- strings:
  - [x] `string?`
  - [x] `string=?`
  - [x] `string<?`
  - [x] `string>?`
  - [x] `string<=?`
  - [x] `string>=?`
  - [x] `make-string`
  - [x] `string`
  - [x] `string-length`
  - [x] `string-ref`
  - [x] `string-set!`
  - [x] `string-upcase`
  - [x] `string-downcase`
  - [x] `string-foldcase`
  - [x] `substring`
  - [x] `string-append`
  - [x] `string->list`
  - [x] `list->string`
  - [x] `string-copy`
  - [x] `string-copy!`
  - [x] `string-fill!`
- controls:
  - [x] `procedure?`
  - [x] `apply`
  - [x] `eval`
  - [x] `map`
  - [x] `string-map`
  - [x] `for-each`
  - [x] `string-for-each`
- ports:
  - [x] `port?`
  - [x] `call-with-input-file`
  - [x] `call-with-output-file`
  - [x] `with-input-from-file`
  - [x] `with-output-to-file`
  - [x] `input-port?`
  - [x] `output-port?`
  - [x] `current-input-port`
  - [x] `current-output-port`
  - [x] `open-input-file`
  - [x] `open-output-file`
  - [x] `close-input-port`
  - [x] `close-output-port`
  - [x] `eof-object?`
  - [x] `eof-object`
- input/output:
  - [x] `read`
  - [x] `read-char`
  - [x] `read-string`
  - [x] `write`
  - [x] `write-char`
  - [x] `write-string`
  - [x] `display`
  - [x] `newline`
- system interface:
  - [x] `load`
  - [x] `file-exists?`
  - [x] `delete-file`
  - [x] `command-line`
  - [x] `exit`
  - [x] `get-environment-variable`
  - [x] `get-environment-variables`
  - [x] `current-second`

## Extensions

- pattern matching with `match`
- lazy evaluation:
  - [x] `delay`
  - [x] `force`
  - [x] `lazy-car`
  - [x] `lazy-cdr`
  - [x] `lazy-cons`
  - [x] `lazy-map`
  - [x] `lazy-filter`
  - [x] `lazy-ref`
  - [x] `head`
