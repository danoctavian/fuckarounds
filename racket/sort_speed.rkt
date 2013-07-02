#lang racket
; sorting 10 mil nums in racket -time eval 
(define (get_rands n)
  (for/list ([i  (in-range 1 n)]) (random 100000)) 
)

(let* ([nums (get_rands 10000000)]
       [start (current-inexact-milliseconds)])
  (displayln start)
  (sort nums <)
  (displayln (- (current-inexact-milliseconds) start))
)
