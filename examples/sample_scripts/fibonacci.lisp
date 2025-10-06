#!/usr/bin/env cracked
; Fibonacci sequence generator

(define (fib n)
  (if (<= n 1)
      n
      (+ (fib (- n 1))
         (fib (- n 2)))))

; Print first 10 fibonacci numbers
(define (print-fibs n)
  (if (> n 0)
      (begin
        (println (fib (- 10 n)))
        (print-fibs (- n 1)))
      #f))

(print-fibs 10)
