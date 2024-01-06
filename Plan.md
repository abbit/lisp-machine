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

## Реализация на Rust должна содержать:

### Базовые типы данных

- [x] `boolean` (#t, #f, #true, #false)
- [x] `number` (integer, float)
- [x] `string`
- [x] `symbol`
- [x] `list`
- [x] `pair` (`improper list`)
- [x] `char`
- [x] `procedure`
- [ ] `port`

### Стандартная библиотека

- [ ] equivalence predicates:
	- [ ] `eqv?`
	- [ ] `eq?`
	- [ ] `equal?`
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
	- [ ] `caar`
	- [ ] `cadr`
	- [ ] `cdar`
	- [ ] `cddr`
	- [ ] `caaar`, `caadr`, ..., `cddar`, `cdddr`
	- [ ] `null?`
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
- [ ] strings:
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
	- [ ] `string->list`
	- [ ] `list->string`
	- [x] `string-copy`
    - [x] `string-copy!`
    - [x] `string-fill!`
- [ ] controls:
	- [ ] `procedure?`
	- [x] `apply`
	- [x] `eval`
	- [ ] `map`
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

### Специальные формы

- [x] `define`
- [x] `define-macro`
- [x] `set!`
- [x] `lambda`
- [x] `if`
- [x] `quote`
- [x] `quasiquote`
- [x] `unquote`
- [x] `unquote-splicing`
- [x] `include`
- [x] `load`

## Специальные формы, которые должны быть реализованы на языке Scheme

- [ ] `let`
- [ ] `let*`
- [ ] `letrec`
- [ ] `letrec*`
- [ ] `let-values`
- [ ] `let*-values`
- [ ] `cond`
- [ ] `when`
- [ ] `unless`
- [ ] `cond-expand`
- [ ] `do`
- [ ] `named let`

## Дополнительные требования:

- [ ] Реализация pattern matching
- [ ] Реализация ленивых вычислений, отложенного порядка вычислений для вызовов функций