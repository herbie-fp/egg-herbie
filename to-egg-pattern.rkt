#lang racket

(provide to-egg-pattern constant?)

(module+ test (require rackunit))

;; copy of constants from herbie/fpcore for egg-herbie
(define constants
  '(E LOG2E LOG10E LN2 LN10
      PI PI_2 PI_4 1_PI 2_PI 2_SQRTPI
      SQRT2 SQRT1_2 MAXFLOAT HUGE_VAL
      TRUE FALSE))

(define (constant? x)
  (set-member? constants x))

(define (to-egg-pattern datum)
  (cond
    [(list? datum)
     (string-join
      (cons
       (symbol->string (first datum))
       (map (lambda (sub-expr) (to-egg-pattern sub-expr))
            (rest datum)))
      " "
      #:before-first "("
      #:after-last ")")]
    [(symbol? datum)
     (format (if (constant? datum) "~a" "?~a") datum)]
    [(number? datum)
     (number->string datum)]
    [else
     (error "expected list, number, or symbol")]))

(module+ test
  (check-equal? (to-egg-pattern `(+ a b)) "(+ ?a ?b)")
  (check-equal? (to-egg-pattern `(/ c (- 2 a))) "(/ ?c (- 2 ?a))"))
