(define (inc-seq x)
  (display x)
  (newline)
  (inc-seq (+ x 1)))

(inc-seq 1)
