#lang racket

(require ffi/unsafe ffi/unsafe/define racket/runtime-path)

(require "./to-egg-pattern.rkt" "./egg-interface.rkt")

(module+ test (require rackunit))

(provide egraph-run egraph-add-exprs egraph-run-iter
         egraph-get-simplest egg-expr->expr egg-add-exn?
         make-ffi-rules free-ffi-rules egraph-get-cost egraph-get-size)

;; the first hash table maps all symbols and non-integer values to new names for egg
;; the second hash is the reverse of the first
(struct egraph-data (egraph-pointer egg->herbie-dict herbie->egg-dict))
;; interface struct for accepting rules
(struct irule (name input output) #:prefab)

(define (egraph-get-size egraph-data)
  (egraph_get_size (egraph-data-egraph-pointer egraph-data)))

(define (egraph-get-cost egraph-data node-id)
  (egraph_get_cost (egraph-data-egraph-pointer egraph-data) node-id))

(define (egraph-get-simplest egraph-data node-id)
  (egraph_get_simplest (egraph-data-egraph-pointer egraph-data) node-id))

(define (make-raw-string s)
  (define b (string->bytes/utf-8 s))
  (define n (bytes-length b))
  (define ptr (malloc 'raw (+ n 1)))
  (memcpy ptr b n)
  (ptr-set! ptr _byte n 0)
  ptr)

(define (make-ffi-rules rules)
  (for/list [(rule rules)]
    (define name (make-raw-string (symbol->string (irule-name rule))))
    (define left (make-raw-string (to-egg-pattern (irule-input rule))))
    (define right (make-raw-string (to-egg-pattern (irule-output rule))))
    (make-FFIRule name left right)))

(define (free-ffi-rules rules)
  (for [(rule rules)]
    (free (FFIRule-name rule))
    (free (FFIRule-left rule))
    (free (FFIRule-right rule))
    (free rule)))

(define (egraph-run-iter egraph-data node-limit ffi-rules precompute?)
  (egraph_run_iter (egraph-data-egraph-pointer egraph-data) node-limit ffi-rules precompute?))

(define (run-rules-recursive egraph-data node-limit ffi-rules precompute? last-count)
  (egraph_run_iter (egraph-data-egraph-pointer egraph-data) node-limit ffi-rules precompute?)
  (define cnt (egraph_get_size (egraph-data-egraph-pointer egraph-data)))
  (if (and (< cnt node-limit) (> cnt last-count))
      (run-rules-recursive egraph-data node-limit ffi-rules precompute? cnt)
      (void)))

(define (egraph-run-rules egraph-data node-limit rules precompute?)
  (run-rules-recursive egraph-data node-limit (make-ffi-rules rules) precompute? 0))


;; calls the function on a new egraph, and cleans up
(define (egraph-run egraph-function)
  (define egraph (egraph-data (egraph_create) (make-hash) (make-hash)))
  (define res (egraph-function egraph))
  (egraph_destroy (egraph-data-egraph-pointer egraph))
  res)

(define (egg-expr->expr expr eg-data)
  (define parsed (read (open-input-string expr)))
  (egg-parsed->expr parsed (egraph-data-egg->herbie-dict eg-data)))

(define (egg-parsed->expr parsed rename-dict)
  (match parsed
    [(list first-parsed second-parsed rest-parsed ...)
     (if (constant? first-parsed)
         (if (equal? second-parsed 'real)  ; parametrized constants
               first-parsed
               (string->symbol (string-append (~s first-parsed) "." (~s second-parsed))))
         (cons                             ; parameterized operators
           (if (equal? second-parsed 'real)
               first-parsed
               (string->symbol (string-append (~s first-parsed) "." (~s second-parsed))))
           (map (curryr egg-parsed->expr rename-dict) rest-parsed)))]
    [(or (? number?) (? constant?))
     parsed]
    [else
     (hash-ref rename-dict parsed)]))

(define (parameterized-constant? sym)
  (define str (symbol->string sym))
  (match (regexp-match #px"([^\\s^\\.]+)\\.([^\\s]+)" str)
    [(list _ constant prec) #t]
    [_ #f]))

;; returns a pair of the string representing an egg expr, and updates the hash tables in the egraph
(define (expr->egg-expr expr egg-data)
  (define egg->herbie-dict (egraph-data-egg->herbie-dict egg-data))
  (define herbie->egg-dict (egraph-data-herbie->egg-dict egg-data))
  (expr->egg-expr-helper expr egg->herbie-dict herbie->egg-dict))

(define (expr->egg-expr-helper expr egg->herbie-dict herbie->egg-dict)
  (cond
    [(list? expr)
     (string-join
      (append
       (extract-operator (first expr))
       (map (lambda (e) (expr->egg-expr-helper e egg->herbie-dict herbie->egg-dict))
            (rest expr)))
      " "
      #:before-first "("
      #:after-last ")")]
    [(and (number? expr) (exact? expr) (real? expr))
     (number->string expr)]
    [(constant? expr)
     (symbol->string expr)]
    [(parameterized-constant? expr)
     (extract-symbol expr)]
    [(hash-has-key? herbie->egg-dict expr)
     (symbol->string (hash-ref herbie->egg-dict expr))]
    [else
     (define new-key (format "h~a" (number->string (hash-count herbie->egg-dict))))
     (define new-key-symbol (string->symbol new-key))
         
     (hash-set! herbie->egg-dict
                expr
                new-key-symbol)
     (hash-set! egg->herbie-dict
                new-key-symbol
                expr)
      new-key]))

(struct egg-add-exn exn:fail ())

;; result function is a function that takes the ids of the nodes
;; egraph-add-exprs returns the result of result-function
(define (egraph-add-exprs eg-data exprs result-function)
  (define egg-exprs
    (map
     (lambda (expr) (expr->egg-expr expr eg-data))
     exprs))
    
  #;
  (debug #:from 'simplify (format "Sending expressions to egg_math:\n ~a"
                                  (string-join egg-exprs "\n ")))
      
  (define expr-results
    (map
     (lambda (expr)
       (egraph_add_expr (egraph-data-egraph-pointer eg-data) expr))
     egg-exprs))
  
  (define node-ids
    (for/list ([result expr-results])
      (if (EGraphAddResult-successp result)
          (EGraphAddResult-id result)
          (raise (egg-add-exn
               (string-append "Failed to add expr to egraph")
               (current-continuation-marks))))))

  (define res (result-function node-ids))

  (for/list ([result expr-results])
    (egraph_addresult_destroy result))

  res)


(module+ test

  (define test-exprs
    (list (cons '(+ y x) "(+ real h0 h1)")
          (cons '(+ x y) "(+ real h1 h0)")
          (cons '(- 2 (+ x y)) "(- real 2 (+ real h1 h0))")
          (cons '(- z (+ (+ y 2) x)) "(- real h2 (+ real (+ real h0 2) h1))")
          (cons '(cos.f64 PI.f64) "(cos f64 (PI f64))")))
  
  (define outputs
   (egraph-run
    (lambda (egg-graph)
      (for/list ([expr-pair test-exprs])
        (expr->egg-expr (car expr-pair) egg-graph)))))
  
  (for ([expr-pair test-exprs]
        [output outputs])
    (check-equal? (cdr expr-pair) output))
    
  
  (define extended-expr-list
    (list
     '(/ (- (exp x) (exp (- x))) 2)
     '(/ (+ (- b) (sqrt (- (* b b) (* (* 3 a) c)))) (* 3 a))
     '(/ (+ (- b) (sqrt (- (* b b) (* (* 3 a) c)))) (* 3 a)) ;; duplicated to make sure it would still work
     '(* r 30)
     '(* 23/54 r)
     '(+ 3/2 1.4)))

  (define extended-results
   (egraph-run
    (lambda (egg-graph)
      (for/list ([expr
                  extended-expr-list])
        (egg-expr->expr (expr->egg-expr expr egg-graph) egg-graph)))))
  (for ([res extended-results] [expected extended-expr-list])
    (check-equal? res expected))
  )
