(define (factorial n)
    (define (factorial-inner n total)
        (if (= n 0)
            total
            (factorial-inner (- n 1) (* total n))))
    (factorial-inner n 1))

(display (factorial 20))
