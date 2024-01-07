# Lisp-машина

## Авторы

- Даниил Клюс 23223
- Михаил Копылов 23223

## Цель

Разработать интерпретатор языка Scheme (части стандарта R7RS) на языке Rust.

## Базовые требования

- [x] Определение функции
- [x] Определение статического лексического контекста
- [x] Рекурсия (в том числе оптимизированная хвостовая рекурсия)
- [x] Ветвление, логические связки с отложенным порядком исполнения
- [x] Присваивание для ранее определенных символов
- [x] Макросы (как defmacro из Clojure)
- [ ] Ввод-вывод (консольный и файловый)
- [ ] Возможность добавления встроенных функций на Rust

Поведение языка должно соответствовать стандарту R7RS.

Реализация интерпретатора на языке Rust должна быть выполнена в виде библиотеки, которая может быть использована в других проектах.

Реализация на Rust должна содержать минимальный набор встроенных функций, необходимых для написания программ на языке Scheme. Все остальные необходимые элементы языка должны быть выражены в этом минимальном базисе.

## Базовые типы данных

- [x] `boolean` (#t, #f, #true, #false)
- [x] `number` (integer, float)
- [x] `string`
- [x] `symbol`
- [x] `list`
- [x] `pair` (`improper list`)
- [x] `char`
- [x] `procedure`
- [ ] `port`

## Специальные формы

### Реализация на Rust должна содержать:

- [x] `define`
- [x] `define-macro`
- [x] `set!`
- [x] `lambda`
- [x] `let` (and named let)
- [x] `letrec`
- [x] `if`
- [x] `cond`
- [x] `and`
- [x] `or`
- [ ] `do`
- [x] `quote`
- [x] `quasiquote`
- [x] `unquote`
- [x] `unquote-splicing`
- [x] `include`
- [x] `load`

### Реализация на Scheme должна содержать:

- [x] `let*`
- [x] `letrec*`
- [x] `case`
- [x] `when`
- [x] `unless`

## Стандартная библиотека

Функции стандартной библиотеки, которые нельзя выразить через другие реализовываются на Rust.
Остальные должны быть реализованы на Scheme.

- [ ] equivalence predicates:
  - [ ] `eqv?`
  - [ ] `eq?`
  - [ ] `equal?`
- [ ] numbers:
  - [ ] `number?`
  - [x] `+`
  - [x] `-`
  - [x] `*`
  - [x] `/`
  - [x] `=`
  - [x] `<`
  - [x] `>`
  - [x] `<=`
  - [x] `>=`
  - [ ] `integer?`
  - [ ] `real?`
  - [ ] `zero?`
  - [ ] `positive?`
  - [ ] `negative?`
  - [ ] `odd?`
  - [ ] `even?`
  - [ ] `abs`
  - [ ] `min`
  - [ ] `max`
  - [ ] `modulo`
  - [ ] `quotient`
  - [ ] `remainder`
  - [ ] `floor`
  - [ ] `ceiling`
  - [ ] `truncate`
  - [ ] `round`
  - [ ] `square`
  - [ ] `sqrt`
  - [ ] `expt`
  - [ ] `number->string`
  - [ ] `string->number`
- [ ] booleans:
  - [ ] `boolean?`
  - [x] `not`
  - [x] `and`
  - [x] `or`
- [ ] pairs and lists:
  - [ ] `pair?`
  - [x] `cons`
  - [x] `car`
  - [x] `cdr`
  - [x] `caar`
  - [x] `cadr`
  - [x] `cdar`
  - [x] `cddr`
  - [x] `caaar`, `caadr`, ..., `cddar`, `cdddr`
  - [x] `null?`
  - [ ] `list?`
  - [ ] `make-list`
  - [x] `list`
  - [ ] `length`
  - [ ] `append`
  - [ ] `reverse`
  - [ ] `list-tail`
  - [ ] `list-ref`
  - [ ] `memq`
  - [ ] `memv`
  - [ ] `member`
  - [ ] `assq`
  - [ ] `assv`
  - [ ] `assoc`
  - [ ] `list-copy`
- [ ] symbols:
  - [ ] `symbol?`
  - [ ] `symbol->string`
  - [ ] `string->symbol`
- [ ] characters:
  - [ ] `char?`
  - [ ] `char=?`
  - [ ] `char<?`
  - [ ] `char>?`
  - [ ] `char<=?`
  - [ ] `char>=?`
  - [ ] `char-alphabetic?`
  - [ ] `char-numeric?`
  - [ ] `char-whitespace?`
  - [ ] `char-upper-case?`
  - [ ] `char-lower-case?`
  - [ ] `digit-value`
  - [ ] `char->integer`
  - [ ] `integer->char`
  - [ ] `char-upcase`
  - [ ] `char-downcase`
  - [ ] `char-foldcase`
- [ ] strings:
  - [ ] `string?`
  - [ ] `string=?`
  - [ ] `string<?`
  - [ ] `string>?`
  - [ ] `string<=?`
  - [ ] `string>=?`
  - [ ] `make-string`
  - [ ] `string`
  - [ ] `string-length`
  - [ ] `string-ref`
  - [x] `string-set!`
  - [ ] `string-upcase`
  - [ ] `string-downcase`
  - [ ] `string-foldcase`
  - [ ] `substring`
  - [ ] `string-append`
  - [ ] `string->list`
  - [ ] `list->string`
  - [ ] `string-copy`
  - [ ] `string-copy!`
  - [ ] `string-fill!`
- [ ] controls:
  - [ ] `procedure?`
  - [x] `apply`
  - [x] `eval`
  - [x] `map`
  - [ ] `string-map`
  - [ ] `for-each`
  - [ ] `string-for-each`
- [ ] ports:
  - [ ] `port?`
  - [ ] `call-with-port`
  - [ ] `call-with-input-file`
  - [ ] `call-with-output-file`
  - [ ] `input-port?`
  - [ ] `output-port?`
  - [ ] `input-port-open?`
  - [ ] `output-port-open?`
  - [ ] `current-input-port`
  - [ ] `current-output-port`
  - [ ] `current-error-port`
  - [ ] `open-input-file`
  - [ ] `open-output-file`
  - [ ] `close-port`
  - [ ] `close-input-port`
  - [ ] `close-output-port`
  - [ ] `open-input-string`
  - [ ] `open-output-string`
  - [ ] `get-output-string`
  - [x] `read`
  - [ ] `read-char`
  - [ ] `peek-char`
  - [ ] `read-line`
  - [ ] `eof-object?`
  - [ ] `eof-object`
  - [ ] `char-ready?`
  - [ ] `read-string`
  - [ ] `write`
  - [x] `display`
  - [x] `newline`
  - [ ] `write-char`
  - [ ] `write-string`
  - [ ] `flush-output-port`
- [ ] system interface:
  - [x] `load`
  - [ ] `file-exists?`
  - [ ] `delete-file`
  - [ ] `command-line`
  - [x] `exit`
  - [ ] `get-environment-variable`
  - [ ] `get-environment-variables`
  - [ ] `current-second`

## Дополнительные требования:

- [ ] Реализация pattern matching
- [ ] Реализация ленивых вычислений, отложенного порядка вычислений для вызовов функций
