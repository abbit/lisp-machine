(define-macro (unless test . body) `(if ,test #f (begin ,@body)))

(define (not x) (if x #f #t))

