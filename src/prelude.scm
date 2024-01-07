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