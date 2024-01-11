(define (a x)
  (display "a ")
  (display x)
  (newline)
  (b (+ x 1)))

(define (b x)
  (display "b ")
  (display x)
  (newline)
  (a (+ x 1)))

(a 1)
