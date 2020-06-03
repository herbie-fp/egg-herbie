#lang racket

(require ffi/unsafe
         ffi/unsafe/define)
(require racket/runtime-path)

(provide egraph_create egraph_destroy egraph_add_expr
         egraph_addresult_destroy egraph_run_iter egraph_get_simplest
         egraph_get_cost egraph_get_size (struct-out EGraphAddResult)
         (struct-out FFIRule))


(define-runtime-path libeggmath-path
  (build-path "target" "release"
              (case (system-type)
                [(windows) "egg_math"]
                [else "libegg_math"])))


(define-ffi-definer define-eggmath (ffi-lib libeggmath-path))

(define _egraph-pointer (_cpointer 'egraph))

(define-cstruct _EGraphAddResult
  ([id _uint]
   [successp _bool]))

(define-cstruct _FFIRule
  ([name _string/utf-8]
   [left _string/utf-8]
   [right _string/utf-8])
  #:malloc-mode 'raw)


;;  -> a pointer to an egraph
(define-eggmath egraph_create (_fun -> _egraph-pointer))

(define-eggmath egraph_destroy (_fun _egraph-pointer -> _void))

;; egraph pointer, s-expr string -> node number
(define-eggmath egraph_add_expr (_fun _egraph-pointer _string/utf-8 -> _EGraphAddResult-pointer))

(define-eggmath egraph_addresult_destroy (_fun _EGraphAddResult-pointer -> _void))


;; egraph pointer, a node limit, a pointer to an array of ffirule, a bool for constant folding, and the size of the ffirule array
(define-eggmath egraph_run_iter (_fun _egraph-pointer
                                      _uint
                                      (ffi-rules : (_list i _FFIRule-pointer))
                                      _bool
                                      (_uint = (length ffi-rules))
                                      -> _void))

;; node number -> s-expr string
(define-eggmath egraph_get_simplest (_fun _egraph-pointer _uint -> _string/utf-8))

(define-eggmath egraph_get_cost (_fun _egraph-pointer _uint -> _uint))
(define-eggmath egraph_get_size (_fun _egraph-pointer -> _uint))
