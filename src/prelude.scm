(define eq? eqv?)

(define (not x) (if x #f #t))

(define (zero? x) (= x 0))

(define (list . args) args)

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
                      (if (eq? (car clause) 'else)
                        (if (eq? (cadr clause) '=>)
                          `(,tmp => ,(caddr clause))
                          `(,tmp ,@(cdr clause)))
                        `((if (memv ,tmp ',(car clause)) ,tmp #f) ,@(cdr clause))))
                    clauses)))))
