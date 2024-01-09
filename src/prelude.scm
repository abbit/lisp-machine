; equals
(define (not x) (if x #f #t))

; math
(define (zero? x) (= 0 x))
(define (positive? x) (> x 0))
(define (negative? x) (< x 0))
(define (even? x)
  (and (integer? x)
       (zero? (modulo x 2))))
(define (odd? x)
  (and (integer? x)
       (not (zero? (modulo x 2)))))

; bool
(define (boolean? x) (if (eq? x #t) #t (eq? x #f)))

; lists and pairs
(define (caar x) (car (car x)))
(define (cadr x) (car (cdr x)))
(define (cdar x) (cdr (car x)))
(define (cddr x) (cdr (cdr x)))
(define (caaar x) (car (car (car x))))
(define (caadr x) (car (car (cdr x))))
(define (cadar x) (car (cdr (car x))))
(define (caddr x) (car (cdr (cdr x))))
(define (cdaar x) (cdr (car (car x))))
(define (cdadr x) (cdr (car (cdr x))))
(define (cddar x) (cdr (cdr (car x))))
(define (cdddr x) (cdr (cdr (cdr x))))
(define (caaaar x) (car (car (car (car x)))))
(define (caaadr x) (car (car (car (cdr x)))))
(define (caadar x) (car (car (cdr (car x)))))
(define (caaddr x) (car (car (cdr (cdr x)))))
(define (cadaar x) (car (cdr (car (car x)))))
(define (cadadr x) (car (cdr (car (cdr x)))))
(define (caddar x) (car (cdr (cdr (car x)))))
(define (cadddr x) (car (cdr (cdr (cdr x)))))
(define (cdaaar x) (cdr (car (car (car x)))))
(define (cdaadr x) (cdr (car (car (cdr x)))))
(define (cdadar x) (cdr (car (cdr (car x)))))
(define (cdaddr x) (cdr (car (cdr (cdr x)))))
(define (cddaar x) (cdr (cdr (car (car x)))))
(define (cddadr x) (cdr (cdr (car (cdr x)))))
(define (cdddar x) (cdr (cdr (cdr (car x)))))
(define (cddddr x) (cdr (cdr (cdr (cdr x)))))

(define (null? x) (equal? x '()))

(define (list? x)
  (if (pair? x)
      (list? (cdr x))
      (null? x)))

(define (length ls)
  (define (length* ls acc)
    (if (null? ls)
        acc
        (length* (cdr ls) (+ 1 acc))))
  (length* ls 0))

(define (acc-reverse l acc)
  (if (null? l)
      acc
      (acc-reverse (cdr l) (cons (car l) acc))))
(define (reverse l)
  (acc-reverse l '()))

(define (append2 l res)
  (if (null? l)
      res
      (append2 (cdr l) (cons (car l) res))))
(define (append-helper ls res)
  (if (null? ls)
      res
      (append-helper (cdr ls) (append2 (reverse (car ls)) res))))
(define (append . o)
  (if (null? o)
      '()
      ((lambda (lol)
         (append-helper (cdr lol) (car lol)))
       (reverse o))))

(define (list-tail l k)
  (if (zero? k)
      l
      (list-tail (cdr l) (- k 1))))
(define (list-ref l k)
  (car (list-tail l k)))

(define (mem predicate obj ls)
  (if (null? ls)
      #f
      (if (predicate obj (car ls))
          ls
          (mem predicate obj (cdr ls)))))
(define (memq obj ls)
  (mem eq? obj ls))
(define (memv obj ls)
  (mem eqv? obj ls))
(define (member obj ls)
  (mem equal? obj ls))

 (define (map1 proc lst)
  (if (null? lst)
    '()
    (cons (proc (car lst))
          (map1 proc (cdr lst)))))
(define (map proc . lists)
  (if (null? (car lists))
    '()
    (cons (apply proc (map1 car lists))
          (apply map proc (map1 cdr lists))))) 

(define (ass predicate obj ls)
  (if (null? ls)
      #f
      (if (predicate obj (caar ls))
          (car ls)
          (ass predicate obj (cdr ls)))))
(define (assq obj ls)
  (ass eq? obj ls))
(define (assv obj ls)
  (ass eqv? obj ls))
(define (assoc obj ls)
  (ass equal? obj ls))

; chars
(define (char=? . c)
  (apply = (map char->integer c)))
(define (char<? . c)
  (apply < (map char->integer c)))
(define (char>? . c)
  (apply > (map char->integer c)))
(define (char<=? . c)
  (apply <= (map char->integer c)))
(define (char>=? . c)
  (apply >= (map char->integer c)))

; strings
(define (string-copy string)
  (substring string 0 (string-length string)))
(define (list->string lst)
  (apply string lst))

; special forms
(define-macro (when test . body) `(if ,test (begin ,@body)))
(define-macro (unless test . body) `(if (not ,test) (begin ,@body)))

(define-macro (let* bindings . body)
  (if (null? bindings)
      `(let ,bindings ,@body)
      `(let ,(list (car bindings)) (let* ,(cdr bindings) ,@body))))

(define-macro (letrec* bindings . body) `(letrec ,bindings ,@body))

(define-macro (case key . clauses)
  (let ((tmp (gensym)))
    `(let ((,tmp ,key))
       (cond ,@(map (lambda (clause)
                      `((memv ,tmp ',(car clause)) ,(cadr clause)))
                    clauses)))))