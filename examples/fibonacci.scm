(define (sub1 n) (- n 1))

(define (fib n)
  (if (= n 0)
    0
    (if (= n 1)
      1
      (+ (fib (sub1 n)) (fib (- n 2))))))

(display (fib 28))
