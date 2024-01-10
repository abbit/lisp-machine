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
- [x] Возможность добавления встроенных функций на Rust

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
- [x] `do`
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

- [x] equivalence predicates:
	- [x] `eqv?`
	- [x] `eq?`
	- [x] `equal?`
- [x] numbers:
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
- [x] booleans:
	- [x] `boolean?`
	- [x] `not`
    - [x] `and`
    - [x] `or`
- [x] pairs and lists:
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
- [x] symbols:
	- [x] `symbol?`
	- [x] `symbol->string`
	- [x] `string->symbol`
- [x] characters:
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
- [x] strings:
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
- [ ] controls:
	- [x] `procedure?`
	- [x] `apply`
	- [x] `eval`
	- [x] `map`
	- [x] `string-map`
	- [x] `for-each`
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