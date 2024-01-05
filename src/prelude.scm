; equals
(define (not x) (if x #f #t))

; math
(define (zero? x) (= 0 x))
(define (positive? x) (> x 0))
(define (negative? x) (< x 0))

; bool

; lists and pairs

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
