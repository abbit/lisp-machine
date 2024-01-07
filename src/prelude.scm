(define eq? eqv?)

(define (caar x) (car (car x)))
(define (cadr x) (car (cdr x)))

(define (not x) (if x #f #t))
(define-macro (when test . body) `(if ,test (begin ,@body)))
(define-macro (unless test . body) `(if (not ,test) (begin ,@body)))

(define (list . args) args)

(define (-map-single proc lst)
  (if (null? lst)
    '()
    (cons (proc (car lst))
          (-map-single proc (cdr lst)))))

(define (map proc . lists)
  (if (null? (car lists))
    '()
    (cons (apply proc (-map-single car lists))
          (apply map proc (-map-single cdr lists)))))

(define-macro (let bindings . body)
  (if (null? bindings)
      `(begin ,@body)
      `((lambda ,(map car bindings) ,@body)
        ,@(map cadr bindings))))

