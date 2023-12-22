(define (factorial-inner x acc)
  (if (= x 0)
    acc
    (factorial-inner (- x 1) (* acc x))))

(define (factorial x)
  (factorial-inner x 1))

(display (factorial 5))
